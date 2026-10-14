use isahc::{AsyncReadResponseExt, RequestExt, ResponseExt, config::Configurable};
use mlua::ExternalResult;
use url::Url;
use vara_util::table_builder::TableBuilder;

use crate::{httprequest::HttpRequest, httpresponse::HttpResponse};

#[derive(mlua::UserData)]
pub struct HttpClient{
    #[lua(set,name="Timeout")]
    timeout: u64,
    #[lua(set,name="Headers")]
    headers: mlua::Table,
}

#[mlua::userdata_impl]
impl HttpClient {
    #[lua(skip)]
    pub fn new(context : &mlua::Lua) -> mlua::Result<Self>{

        Ok(Self{
            timeout: 100,
            headers: context.create_table()?,
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
    fn create_request<'a>(&self,request: &'a HttpRequest) -> mlua::Result<isahc::Request<&'a [u8]>> {
        let body : &'a [u8] = &request.body.as_slice();
        let url = &request.url;
        let method = &request.method.trim().to_ascii_uppercase();

        let mut parsed_uri = Url::parse(url).into_lua_err()?;
        let mut query_uri = parsed_uri.query_pairs_mut();

        for pair in request.query.pairs::<String,String>() {
            let (key,value) = pair?;
            query_uri.append_pair(&key, &value);
        }

        let uri : http::Uri = query_uri.finish()
            .as_str()
            .parse()
            .into_lua_err()?;

        let mut request_builder = isahc::Request::builder()
        .method(method.as_str())
        .timeout(std::time::Duration::from_secs(self.timeout))
        .uri(uri);
    
        for pair in self.headers.pairs::<String,String>() {
            let (key,value) = pair?;
            request_builder = request_builder.header(key, value);
        }

        request_builder.body(body).into_lua_err()
    }

    #[lua(name="SendAsync")]
    pub async fn send(&self,context : &mlua::Lua,request: &HttpRequest) -> mlua::Result<HttpResponse>{
        let request = self.create_request(&request)?;

        let mut response = request.send_async().await.into_lua_err()?;

        self.parse_response(context,&mut response).await
    }

    #[lua(name="AddHeader")]
    pub fn add_header(this: mlua::AnyUserData,key : &str,value : &str) -> mlua::Result<mlua::AnyUserData>{

        {
            let client = this.borrow::<HttpClient>()?;
            client.headers.set(key, value)?;
        }

        Ok(this)
    }
}