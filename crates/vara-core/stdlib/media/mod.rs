use std::{io::Cursor, path::PathBuf};
use mlua::ExternalError;
use vara_util::table_builder::TableBuilder;

mod image_obj;
mod image_format;

fn create_image(_ : &mlua::Lua,data: mlua::Value) -> mlua::Result<image_obj::Image>{
    match data {
        mlua::Value::String(s) => {
            let bytes = crate::stdlib::file::read_file_as_binary(&PathBuf::from(s.to_string_lossy()))?;

            let img = image::ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()?
                .decode()
                .map_err(|err| err.into_lua_err())?;

            Ok(image_obj::Image::new(img.to_rgba8().into_raw(), img.width(), img.height()))
        },
        mlua::Value::Buffer(b) => {
            let bytes = b.to_vec();
            Ok(image_obj::Image::new(bytes, b.len() as u32,1))
        }
        _ => {
            return Err(mlua::Error::FromLuaConversionError {
                from: data.type_name(),
                to: "string or buffer".to_string(),
                message: Some("expected string or buffer".to_string()),
            });
        }
    }
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("image", create_image)?
    .build_readonly()
}