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

pub fn from_oklch(_: &mlua::Lua,(lightness,chroma,hue,alpha):(f32,f32,f32,Option<f32>)) -> mlua::Result<colour::Colour4>{
    let alpha = alpha.unwrap_or(1.0);

    // convert oklch to oklab via polar coordinates
    let a = chroma * hue.to_radians().cos();
    let b = chroma * hue.to_radians().sin();

    // convert from oklab -> lms
    let mut l = lightness + 0.3963377774f32 * a + 0.2158037573f32 * b;
    let mut m = lightness - 0.1055613458f32 * a - 0.0638541728f32 * b;
    let mut s = lightness - 0.0894841775f32 * a - 1.2914855480f32 * b;

    l = l.powi(3); 
    m = m.powi(3); 
    s = s.powi(3); 

    // lms -> linear rgb
    let red = 4.0767416621f32 * l - 3.3077115913f32 * m + 0.2309699292f32 * s;
    let green = -1.2684380046f32 * l + 2.6097574011f32 * m - 0.3413193965f32 * s;
    let blue = -0.0041960863f32 * l - 0.7034186147f32 * m + 1.7076147010f32 * s;

    Ok(colour::Colour4::new(red, green, blue, alpha))
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("New", new)?
    .add_function("FromSRGBA", from_srgba)?
    .add_function("FromHex", from_hex)?
    .add_function("FromHSVA", from_hsva)?
    .add_function("FromOKLCH", from_oklch)?
    .build_readonly()
}
