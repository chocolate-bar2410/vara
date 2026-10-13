#[derive(mlua::UserData)]
pub struct HttpResponse{
    #[lua(get,name="Status")]
    pub status: u16,
    #[lua(get,name="Body")]
    pub body: Vec<u8>,
    #[lua(get,name="Ok")]
    pub ok: bool,
    #[lua(get,name="Headers")]
    pub headers: mlua::Table,
    #[lua(get,name="Url")]
    pub url: String,
}

#[mlua::userdata_impl]
impl HttpResponse {
    #[lua(meta,name="__tostring")]
    fn tostring(&self) -> mlua::Result<String>{

        let result = format!("http response {} {}",self.status,self.url);

        Ok(result)
    }
}
