use vara_util::table_builder::TableBuilder;

mod gtkcontext;

pub fn create_qt_context(_: &mlua::Lua,():()) -> mlua::Result<()>{
    Ok(())
}

pub fn get_module(context : &mlua::Lua) -> mlua::Result<mlua::Table>{

    TableBuilder::new(context.clone())?
    .add_function("CreateGtkContext", create_ui_context)?
    .build_readonly()
}