use std::{io::Cursor, path::PathBuf};
use mlua::ExternalError;
use vara_util::table_builder::TableBuilder;

use crate::stdlib as vara_stdlib;

mod image_obj;
mod image_format;
mod image_interpolation;

fn create_image(_ : &mlua::Lua,data: mlua::Value) -> mlua::Result<image_obj::Image>{
    match data {
        mlua::Value::String(s) => {
            let bytes = vara_stdlib::file::read_file_as_binary(&PathBuf::from(s.to_string_lossy()))?;

            let reader = image::ImageReader::new(Cursor::new(bytes))
                .with_guessed_format()?;

            let format = reader.format();

            let img = reader
                .decode()
                .map_err(|err| err.into_lua_err())?;

            let data = match format {
                Some(image::ImageFormat::Jpeg) => img.to_rgb8().into_raw(),
                _ => img.to_rgba8().into_raw(),
            };

            let colour_channels = match format {
                Some(image::ImageFormat::Jpeg) => 3,
                _ => 4,
            };

            Ok(image_obj::Image::new(data, img.width(), img.height(),colour_channels))
        },
        mlua::Value::Buffer(b) => {
            let bytes = b.to_vec();
            Ok(image_obj::Image::new(bytes, b.len() as u32,1,4))
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