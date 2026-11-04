pub fn print(_context: &mlua::Lua, params : mlua::MultiValue) -> mlua::Result<()>{
    println!("{} parameters given",params.len());
    Ok(())
}

pub fn warn(_context: &mlua::Lua, params : mlua::MultiValue) -> mlua::Result<()>{
    println!("\x1b[33m{} parameters given\x1b[0m",params.len());
    Ok(())
}