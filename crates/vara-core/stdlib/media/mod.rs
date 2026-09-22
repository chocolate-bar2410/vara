use std::io::Cursor;
use mlua::ExternalError;
use vara_util::table_builder::TableBuilder;

mod image_obj;
mod image_format;

fn decode_image(_ : &mlua::Lua,(format,data): (String,mlua::Value)) -> mlua::Result<image_obj::Image>{
    let bytes = match data {
        mlua::Value::String(s) => s.as_bytes().to_vec(),
        mlua::Value::Buffer(b) => b.to_vec(),
        _ => {
            return Err(mlua::Error::FromLuaConversionError {
                from: data.type_name(),
                to: "string or buffer".to_string(),
                message: Some("expected string or buffer".to_string()),
            });
        }
    };

    let img_format = image_format::image_format_fromstring(&format)?;
    let mut image_reader = image::ImageReader::new(Cursor::new(bytes));

    image_reader.set_format(img_format);

    let img = image_reader.decode().map_err(|err| err.into_lua_err())?;

    Ok(image_obj::Image::new(img.to_rgba8().into_raw(), img.width(), img.height()))
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("decodeImage", decode_image)?
    .build_readonly()
}