
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
        Ok(())
    }

}