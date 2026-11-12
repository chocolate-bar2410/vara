use vara_util::table_builder::TableBuilder;

use httpclient::HttpClient;

use httprequest::HttpRequest;

mod httpclient;
mod httpresponse;
mod httprequest;

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("HttpClient", |lua: &mlua::Lua, ()| Ok(HttpClient::new(&lua)))?
        .add_function("HttpRequest", |lua: &mlua::Lua, url: String| Ok(HttpRequest::new(&lua,url)))?
        .build_readonly()
}