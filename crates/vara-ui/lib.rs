use vara_util::table_builder::TableBuilder;

mod window;
mod ui_context;

pub fn get_modules(context : &mlua::Lua) -> mlua::Result<mlua::Table>{

    TableBuilder::new(context.clone())?
    .build_readonly()
}