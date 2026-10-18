use std::io::Write;

use vara_util::table_builder::TableBuilder;

fn clear(_: &mlua::Lua,_: ()) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();

    write!(stdout,"\x1b[2J\x1b[H\x1b[3J")?;
    stdout.flush()?;

    Ok(())
}

fn hide_cursor(_: &mlua::Lua,_: ()) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();
    write!(stdout,"\x1b[?25l")?;
    stdout.flush()?;

    Ok(())
}

fn show_cursor(_: &mlua::Lua,_: ()) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();
    write!(stdout,"\x1b[?25h")?;
    stdout.flush()?;

    Ok(())
}

fn move_cursor(_: &mlua::Lua,(x,y): (u32,u32)) -> mlua::Result<()>{
    let mut stdout = std::io::stdout();
    write!(stdout,"\x1b[{};{}H",y,x)?;
    stdout.flush()?;

    Ok(())
}


pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {

    let style_tab = TableBuilder::new(context.clone())?
        .add_values(vec![
            ("Reset",           "\x1b[0m"),
            ("Bold",            "\x1b[1m"),
            ("Dim",             "\x1b[2m"),
            ("Italic",          "\x1b[3m"),
            ("Underline",       "\x1b[4m"),
            ("Blink",           "\x1b[5m"),
            ("Invert",          "\x1b[7m"),
            ("Hidden",          "\x1b[8m"),
            ("StrikeThrough",   "\x1b[9m"),
        ])?.build_readonly()?;

    let colour_tab = TableBuilder::new(context.clone())?
        .add_values(vec![
            ("Black",   "\x1b[30m"),
            ("Red",     "\x1b[31m"),
            ("Green",   "\x1b[32m"),
            ("Yellow",  "\x1b[33m"),
            ("Blue",    "\x1b[34m"),
            ("Magenta", "\x1b[35m"),
            ("Cyan",    "\x1b[36m"),
            ("White",   "\x1b[37m"),

            ("BackgroundBlack",   "\x1b[40m"),
            ("BackgroundRed",     "\x1b[41m"),
            ("BackgroundGreen",   "\x1b[42m"),
            ("BackgroundYellow",  "\x1b[43m"),
            ("BackgroundBlue",    "\x1b[44m"),
            ("BackgroundMagenta", "\x1b[45m"),
            ("BackgroundCyan",    "\x1b[46m"),
            ("BackgroundWhite",   "\x1b[47m"),

            ("BrightBlack",   "\x1b[90m"),
            ("BrightRed",     "\x1b[91m"),
            ("BrightGreen",   "\x1b[92m"),
            ("BrightYellow",  "\x1b[93m"),
            ("BrightBlue",    "\x1b[94m"),
            ("BrightMagenta", "\x1b[95m"),
            ("BrightCyan",    "\x1b[96m"),
            ("BrightWhite",   "\x1b[97m"),

            ("BackgroundBrightBlack",   "\x1b[100m"),
            ("BackgroundBrightRed",     "\x1b[101m"),
            ("BackgroundBrightGreen",   "\x1b[102m"),
            ("BackgroundBrightYellow",  "\x1b[103m"),
            ("BackgroundBrightBlue",    "\x1b[104m"),
            ("BackgroundBrightMagenta", "\x1b[105m"),
            ("BackgroundBrightCyan",    "\x1b[106m"),
            ("BackgroundBrightWhite",   "\x1b[107m"),
        ])?.build_readonly()?;

    TableBuilder::new(context.clone())?
        .add_function("Clear", clear)?
        .add_function("HideCursor", hide_cursor)?
        .add_function("ShowCursor", show_cursor)?
        .add_function("MoveCursor", move_cursor)?
        .add_value("Style", style_tab)?
        .add_value("Colour", colour_tab)?
        .build_readonly()
}
