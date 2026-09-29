use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufWriter, Read};
use std::path::PathBuf;

use vara_util::table_builder::TableBuilder;

mod file_obj;
mod directory;

pub fn read_file(path: &PathBuf) -> io::Result<String> {
    let mut file = fs::File::open(path)
        .expect(format!("Could not find file at path {}", path.display()).as_str());
    let mut contents: String = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn read_file_as_binary(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)
        .expect(format!("Could not find file at path {}", path.display()).as_str());
    let mut contents: Vec<u8> = Vec::new();

    file.read_to_end(&mut contents)?;

    Ok(contents)
}

fn read_file_fn(context: &mlua::Lua,arg: mlua::LuaString) -> Result<mlua::Buffer, mlua::error::Error> {
    let path = PathBuf::from(arg.to_str()?.to_string());

    let contents = read_file_as_binary(&path)?;

    context.create_buffer(contents)
}

fn write_file_fn(_: &mlua::Lua,(path_string, data): (mlua::LuaString, mlua::Value)) -> Result<(), mlua::error::Error> {
    let bytes: Vec<u8> = match data {
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

    let path = PathBuf::from(path_string.to_str()?.to_string());
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    writer.write_all(&bytes)?;

    Ok(())
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("read", read_file_fn)?
        .add_function("write", write_file_fn)?
        .add_function("file", |_: &mlua::Lua, path: PathBuf| file_obj::File::new(&path))?
        .add_function("directory", |_: &mlua::Lua, path: PathBuf| directory::Directory::new(&path))?
        .build_readonly()
}
