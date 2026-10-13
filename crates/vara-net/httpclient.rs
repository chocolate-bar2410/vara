use reqwest::Client;
use reqwest::ClientBuilder;

use mlua::ExternalResult;
use url::Url;

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
    pub async fn get(&self,context : &mlua::Lua,url: String) -> mlua::Result<(mlua::Buffer,u16)>{
        
        let url = Url::parse(&url).into_lua_err()?;
        let response = self.client.get(url).send().await.into_lua_err()?;

        let status_code = response.status().as_u16();
        let body = response.bytes().await.into_lua_err()?.to_vec();

        Ok((context.create_buffer(body)?,status_code))
    }

}