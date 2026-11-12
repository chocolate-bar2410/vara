#[derive(mlua::UserData)]
pub struct HttpRequest{
    #[lua(get,set,name="Method")]
    pub method: String,
    #[lua(get,set,name="Query")]
    pub query: mlua::Table,
    #[lua(get,set,name="Url")]
    pub url: String,
    #[lua(skip)]
    pub body: Vec<u8>,
}

#[mlua::userdata_impl]
impl HttpRequest {
    #[lua(skip)]
    pub fn new(context : &mlua::Lua,url : String) -> mlua::Result<Self>{

        Ok(Self{
            method: String::from("GET"),
            query: context.create_table()?,
            body: Vec::new(),
            url: url
        })
    }

    #[lua(name="AddQuery")]
    pub fn add_query(this: mlua::AnyUserData,key : &str,value : &str) -> mlua::Result<mlua::AnyUserData>{

        {
            let client = this.borrow::<HttpRequest>()?;
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