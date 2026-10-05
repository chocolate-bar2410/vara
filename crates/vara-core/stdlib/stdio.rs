use std::io::{self, Read, Write};
use vara_util::table_builder::TableBuilder;

fn write(_: &mlua::Lua, arg: mlua::LuaString) -> Result<(), mlua::error::Error> {
    let message = arg.as_bytes();

    io::stdout().write_all(&message).unwrap();

    Ok(())
}

fn read(
    context: &mlua::Lua,
    chunk_size: Option<usize>,
) -> Result<mlua::LuaString, mlua::error::Error> {
    io::stdout().flush().map_err(mlua::Error::external)?;

    let mut stdin = io::stdin().lock();
    let size = chunk_size.unwrap_or(1024);
    let mut buffer = vec![0; size];

    let bytes_read = stdin.read(&mut buffer).map_err(mlua::Error::external)?;

    buffer.truncate(bytes_read);

    context.create_string(&buffer)
}

fn readline(context: &mlua::Lua, _: ()) -> Result<mlua::LuaString, mlua::error::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    context.create_string(&input.trim())
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("Write", write)?
        .add_function("Read", read)?
        .add_function("ReadLine", readline)?
        .build_readonly()
}
