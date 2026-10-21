use mlua::ExternalError;
use vara_util::table_builder::TableBuilder;

pub mod colour;

pub fn new(_: &mlua::Lua,(red,green,blue,alpha):(f32,f32,f32,Option<f32>)) -> mlua::Result<colour::Colour4>{
    let alpha = alpha.unwrap_or(1.0);

    Ok(colour::Colour4::new(red, green, blue, alpha))
}

pub fn from_srgba(_: &mlua::Lua,(red,green,blue,alpha):(f32,f32,f32,Option<f32>)) -> mlua::Result<colour::Colour4>{
    let alpha = alpha.unwrap_or(1.0);

    Ok(colour::Colour4::from_srgba(red, green, blue, alpha))
}

pub fn from_hex(_: &mlua::Lua,hex_string:String) -> mlua::Result<colour::Colour4>{
    let hex = hex_string.strip_prefix("#").unwrap_or(&hex_string);

    let error_message = || {format!("Tried to convert invalid hex string {} into a Colour4",hex_string).into_lua_err()};

    match hex.len() {
        6 => {
            let red = u8::from_str_radix(&hex[0..2], 16).map_err(|_| error_message())? as f32 / 255.0;
            let green = u8::from_str_radix(&hex[2..4], 16).map_err(|_| error_message())? as f32 / 255.0;
            let blue = u8::from_str_radix(&hex[4..6], 16).map_err(|_| error_message())? as f32 / 255.0;

            Ok(colour::Colour4::from_srgba(red, green, blue, 1.0))
        }

        8 => {
            let red = u8::from_str_radix(&hex[0..2], 16).map_err(|_| error_message())? as f32 / 255.0;
            let green = u8::from_str_radix(&hex[2..4], 16).map_err(|_| error_message())? as f32 / 255.0;
            let blue = u8::from_str_radix(&hex[4..6], 16).map_err(|_| error_message())? as f32 / 255.0;
            let alpha = u8::from_str_radix(&hex[6..8], 16).map_err(|_| error_message())? as f32 / 255.0;

            Ok(colour::Colour4::from_srgba(red, green, blue, alpha))
        }

        _ => {
            return Err("Invalid hexstring length, hex strings must have 6 or 8 characters.".into_lua_err())}
    }
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("New", new)?
    .add_function("FromSRGBA", from_srgba)?
    .add_function("FromHex", from_hex)?
    .build_readonly()
}
