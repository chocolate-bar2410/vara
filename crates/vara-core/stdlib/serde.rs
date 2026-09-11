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
    };

    context.create_string(bytes)
}

pub fn decode(
    context: &mlua::Lua,
    (format, value): (SerdeFormat, mlua::LuaString),
) -> mlua::Result<mlua::Value> {
    match format {
        SerdeFormat::JSON => {
            let serialised: serde_json::Value =
                serde_json::from_slice(&value.as_bytes()).into_lua_err()?;

            context.to_value_with(&serialised, LUAU_SERIALISE_OPTION)
        }
    }
}
