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


pub fn from_hsva(_: &mlua::Lua,(hue,saturation,value,alpha):(f32,f32,f32,Option<f32>)) -> mlua::Result<colour::Colour4>{
    let alpha = alpha.unwrap_or(1.0);

    let hue = (hue / 60.0).rem_euclid(6.0);

    let red = (hue + 5.0).rem_euclid(6.0);
    let green = (hue + 3.0).rem_euclid(6.0);
    let blue = (hue + 1.0).rem_euclid(6.0);

    Ok(colour::Colour4::from_srgba(
        value * (1.0 - saturation * red.min(4.0 - red).clamp(0.0, 1.0)), 
        value * (1.0 - saturation * green.min(4.0 - green).clamp(0.0, 1.0)), 
        value * (1.0 - saturation * blue.min(4.0 - blue).clamp(0.0, 1.0)), 
        alpha
    ))
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("New", new)?
    .add_function("FromSRGBA", from_srgba)?
    .add_function("FromHex", from_hex)?
    .add_function("FromHSVA", from_hsva)?
    .build_readonly()
}
