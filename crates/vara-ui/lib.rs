use vara_util::table_builder::TableBuilder;

mod render_state;
mod ui_context;
use ui_context::UiContext;
mod primitives;
mod rect_pipeline;

pub fn get_module(context : &mlua::Lua) -> mlua::Result<mlua::Table>{

    TableBuilder::new(context.clone())?
    .add_async_function("CreateContext", |_:mlua::Lua,()|async {Ok(UiContext::new())})?
    .build_readonly()
}