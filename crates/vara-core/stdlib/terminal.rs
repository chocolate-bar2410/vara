use std::io::Write;

use vara_util::table_builder::TableBuilder;

fn clear(_: &mlua::Lua,_: ()) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();

    stdout.write_all(b"\x1b[2J\x1b[H\x1b[3J")?;
    stdout.flush()?;

    Ok(())
}

fn hide_cursor(_: &mlua::Lua,_: ()) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();
    stdout.write_all(b"\x1b[?25l")?;
    stdout.flush()?;

    Ok(())
}

fn show_cursor(_: &mlua::Lua,_: ()) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();
    stdout.write_all(b"\x1b[?25h")?;
    stdout.flush()?;

    Ok(())
}

fn move_cursor(_: &mlua::Lua,(x,y): (u32,u32)) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();
    stdout.write_all(&format!("\x1b[{};{}H",y,x).into_bytes())?;
    stdout.flush()?;

    Ok(())
}


pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("Clear", clear)?
        .add_function("HideCursor", hide_cursor)?
        .add_function("ShowCursor", show_cursor)?
        .add_function("MoveCursor", move_cursor)?
        .build_readonly()
}
