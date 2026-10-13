use reqwest::blocking::Client;

use mlua::ExternalResult;
use url::Url;
use vara_util::table_builder::TableBuilder;

use crate::httpresponse::HttpResponse;

#[derive(mlua::UserData)]
pub struct HttpClient{
    #[lua(skip)]
    client: Client,
}

#[mlua::userdata_impl]
impl HttpClient {
    #[lua(skip)]
    pub fn new() -> Self{
        Self{
            client: Client::new()
        }
    }

    #[lua(name="GetAsync")]
    pub async fn get(&self,context : &mlua::Lua,url: String) -> mlua::Result<HttpResponse>{
        
        let url = Url::parse(&url).into_lua_err()?;
        let response = self.client.get(url).send().into_lua_err()?;

        let status_code = response.status().as_u16();

        let mut header_builder = TableBuilder::new(context.clone())?;

        for(key,value) in response.headers(){
            header_builder = header_builder.add_value(key.as_str(), value.to_str().into_lua_err()?)?;
        }

        let headers = header_builder.build()?;
        let result_url = response.url().to_string();


        let result = HttpResponse {
            status: status_code,
            body: response.bytes().into_lua_err()?.to_vec(),
            ok: status_code > 100 && status_code < 300,
            headers: headers,
            url: result_url,
        };


        Ok(result)
    }

}