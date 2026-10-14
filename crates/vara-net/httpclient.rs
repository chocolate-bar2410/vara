use isahc::{AsyncReadResponseExt, RequestExt, ResponseExt, config::Configurable};
use mlua::ExternalResult;
use url::Url;
use vara_util::table_builder::TableBuilder;

use crate::httpresponse::HttpResponse;

#[derive(mlua::UserData)]
pub struct HttpClient{
    #[lua(name="Method")]
    method: String,
    #[lua(name="Timeout")]
    timeout: u64,
    #[lua(name="Headers")]
    headers: mlua::Table,
    #[lua(name="Query")]
    query: mlua::Table,
    #[lua(skip)]
    body: Vec<u8>,

}

#[mlua::userdata_impl]
impl HttpClient {
    #[lua(skip)]
    pub fn new(context : &mlua::Lua) -> mlua::Result<Self>{

        Ok(Self{
            method: String::from("GET"),
            timeout: 100,
            headers: context.create_table()?,
            query: context.create_table()?,
            body: Vec::new(),
        })
    }

    #[lua(skip)]
    async fn parse_response(&self,context : &mlua::Lua,response : &mut isahc::Response<isahc::AsyncBody>) -> mlua::Result<HttpResponse>{
        let status_code = response.status().as_u16();

        let response_headers = response.headers();

        let mut header_builder = TableBuilder::new(context.clone())?;

        for(key,value) in response_headers{
            header_builder = header_builder.add_value(
                key.as_str(), 
                value.to_str()
                     .into_lua_err()?
            )?;
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

    #[lua(skip)]
    fn create_request(&self,url: String) -> mlua::Result<isahc::Request<&[u8]>> {
        let body = &self.body as &[u8];

        let mut parsed_uri = Url::parse(&url).into_lua_err()?;
        let mut query_uri = parsed_uri.query_pairs_mut();

        for pair in self.query.pairs::<String,String>() {
            let (key,value) = pair?;
            query_uri.append_pair(&key, &value);
        }

        let uri : http::Uri = query_uri.finish()
            .as_str()
            .parse()
            .into_lua_err()?;

        let mut request_builder = isahc::Request::builder()
        .method(self.method.as_str().to_ascii_uppercase().trim())
        .timeout(std::time::Duration::from_secs(self.timeout))
        .uri(uri);
    
        for pair in self.headers.pairs::<String,String>() {
            let (key,value) = pair?;
            request_builder = request_builder.header(key, value);
        }

        request_builder.body(body).into_lua_err()
    }

    #[lua(name="RequestAsync")]
    pub async fn request(&self,context : &mlua::Lua,url: String) -> mlua::Result<HttpResponse>{
        let request = self.create_request(url)?;

        let mut response = request.send_async().await.into_lua_err()?;

        self.parse_response(context,&mut response).await
    }

    #[lua(name="AddQuery")]
    pub fn add_query(this: mlua::AnyUserData,key : &str,value : &str) -> mlua::Result<mlua::AnyUserData>{

        {
            let client = this.borrow::<HttpClient>()?;
            client.query.set(key, value)?;
        }

        Ok(this)
    }


    #[lua(getter,name="Body")]
    pub fn get_body(&self,context : &mlua::Lua) -> mlua::Result<mlua::Buffer>{
        let body = &self.body;
        context.create_buffer(body)
    }
}