use mlua::ExternalResult;

#[derive(mlua::UserData)]
pub struct Clipboard{
    #[lua(skip)]
    handle: arboard::Clipboard
}

#[mlua::userdata_impl]
impl Clipboard{

    #[lua(skip)]
    pub fn new() -> mlua::Result<Self>{
        Ok(Self { handle:  arboard::Clipboard::new().into_lua_err()?})
    }

    #[lua(name="GetText")]
    pub fn get_text(&mut self) -> mlua::Result<Option<String>>{
        match self.handle.get_text(){
            Ok(text) => Ok(Some(text)),
            Err(_) => Ok(None)
        }
    }
}