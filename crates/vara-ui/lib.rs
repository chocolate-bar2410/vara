use vara_util::table_builder::TableBuilder;

#[cfg(windows)]
mod win_ui;

#[cfg(windows)]
pub fn create_ui_context(_: &mlua::Lua,():()) -> mlua::Result<win_ui::UiApplication>{
    win_ui::UiApplication::new()
}

pub fn get_module(context : &mlua::Lua) -> mlua::Result<mlua::Table>{

    TableBuilder::new(context.clone())?
    .add_function("CreateContext", create_ui_context)?
    .build_readonly()
}