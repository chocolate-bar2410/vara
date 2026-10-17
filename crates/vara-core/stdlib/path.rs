
use vara_util::table_builder::TableBuilder;

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .build_readonly()
}
