use isahc::{AsyncReadResponseExt, ResponseExt};
use mlua::ExternalResult;
use vara_util::table_builder::TableBuilder;

use crate::httpresponse::HttpResponse;

#[derive(mlua::UserData)]
pub struct HttpClient{
    #[lua(name="Method")]
    method: String,
    #[lua(name="Timeout")]
    timeout: f32,
    #[lua(name="Headers")]
    headers: mlua::Table,
    #[lua(name="Query")]
    query: mlua::Table,
    #[lua(name="Body")]
    body: Vec<u8>,
}

#[mlua::userdata_impl]
impl HttpClient {
    #[lua(skip)]
    pub fn new(context : &mlua::Lua) -> mlua::Result<Self>{
        Ok(Self{
            method: String::from("GET"),
            timeout: 100.0,
            headers: context.create_table()?,
            query: context.create_table()?,
            body: Vec::new()
        })
    }

    #[lua(name="GetAsync")]
    pub async fn get(&self,context : &mlua::Lua,url: String) -> mlua::Result<HttpResponse>{

        let mut response = isahc::get_async(url).await.into_lua_err()?;

        let status_code = response.status().as_u16();

        let response_headers = response.headers();

        let mut header_builder = TableBuilder::new(context.clone())?;

        for(key,value) in response_headers{
            header_builder = header_builder.add_value(key.as_str(), value.to_str().into_lua_err()?)?;
        }

        let headers = header_builder.build_readonly()?;

        let body = response.bytes().await.into_lua_err()?;

        let mut result_url : String = String::from("");

        if let Some(response_url) = response.effective_uri() {
            result_url = response_url.to_string();
        }

        let result = HttpResponse{
            status: status_code,
            body: body,
            ok: status_code >= 200 && status_code < 300,
            headers,
            url: result_url
        };

        Ok(result)
    }

}