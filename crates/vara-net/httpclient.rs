use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;

use mlua::ExternalResult;
use url::Url;

#[derive(mlua::UserData)]
pub struct HttpClient{

}

#[mlua::userdata_impl]
impl HttpClient {
    #[lua(skip)]
    pub fn new() -> Self{
        Self{}
    }

    #[lua(name="SendAsync")]
    pub async fn send(url: String) -> mlua::Result<()>{
        
        let url = Url::parse(&url).into_lua_err()?;
        let uri = url.as_str();

        
        
        Ok(())
    }

}