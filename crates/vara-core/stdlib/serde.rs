use mlua::ExternalError;
use mlua::ExternalResult;
use mlua::LuaSerdeExt;
use mlua::serde::DeserializeOptions;
use mlua::serde::SerializeOptions;

const LUAU_SERIALISE_OPTION: SerializeOptions = SerializeOptions::new()
    .set_array_metatable(false)
    .serialize_none_to_null(false)
    .serialize_unit_to_null(false);

const LUAU_DESERIALISE_OPTION: DeserializeOptions = DeserializeOptions::new()
    .sort_keys(true)
    .deny_recursive_tables(false)
    .deny_unsupported_types(true);

#[derive(Debug, Clone, Copy)]
pub enum SerdeFormat {
    JSON,
    TOML,
    YAML,
}

impl mlua::FromLua for SerdeFormat {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::prelude::LuaResult<Self> {
        let mlua::Value::String(s) = &value else {
            return Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "SerdeFormat".to_string(),
                message: None,
            });
        };

        match s.to_string_lossy().to_ascii_uppercase().trim() {
            "JSON" => Ok(SerdeFormat::JSON),
            "TOML" => Ok(SerdeFormat::TOML),
            "YAML" => Ok(SerdeFormat::YAML),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "SerdeFormat".to_string(),
                message: Some("Invalid format".to_string()),
            }),
        }
    }
}

pub fn encode(
    context: &mlua::Lua,
    (format, value): (SerdeFormat, mlua::Value),
) -> mlua::Result<mlua::LuaString> {
    let bytes = match format {
        SerdeFormat::JSON => {
            let serialised: serde_json::Value =
                context.from_value_with(value, LUAU_DESERIALISE_OPTION)?;
            serde_json::to_vec(&serialised).into_lua_err()?
        }

        SerdeFormat::TOML => {
            let serialised: toml::Value =
                context.from_value_with(value, LUAU_DESERIALISE_OPTION)?;
            toml::to_string(&serialised)
                .into_lua_err()?
                .as_bytes()
                .to_vec()
        }

        SerdeFormat::YAML => {
            // saphyr for some reason needs us to use serde_json values
            let serialised: serde_json::Value =
                context.from_value_with(value, LUAU_DESERIALISE_OPTION)?;

            serde_saphyr::to_string(&serialised)
                .into_lua_err()?
                .into_bytes()
        }
    };

    context.create_string(bytes)
}

pub fn decode(
    context: &mlua::Lua,
    (format, value): (SerdeFormat, mlua::LuaString),
) -> mlua::Result<mlua::Value> {
    let bytes = &value.as_bytes();

    match format {
        SerdeFormat::JSON => {
            let serialised: serde_json::Value = serde_json::from_slice(bytes).into_lua_err()?;

            context.to_value_with(&serialised, LUAU_SERIALISE_OPTION)
        }

        SerdeFormat::TOML => {
            let Ok(s) = String::from_utf8(bytes.to_vec()) else {
                return Err("TOML must be valid utf8".into_lua_err());
            };

            let serialised: toml::Value = toml::from_str(&s).into_lua_err()?;

            context.to_value_with(&serialised, LUAU_SERIALISE_OPTION)
        }

        SerdeFormat::YAML => {
            let serialised: serde_json::Value = serde_saphyr::from_slice(bytes).into_lua_err()?;

            context.to_value_with(&serialised, LUAU_SERIALISE_OPTION)
        }
    }
}
