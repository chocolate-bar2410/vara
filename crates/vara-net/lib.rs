use vara_util::table_builder::TableBuilder;

use httpclient::HttpClient;

mod httpclient;
mod httpresponse;

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("HttpClient", |lua: &mlua::Lua, ()| Ok(HttpClient::new(&lua)))?
        .build_readonly()
}