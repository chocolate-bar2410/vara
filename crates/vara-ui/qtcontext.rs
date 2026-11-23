#[derive(mlua::UserData)]
pub struct QtContext{

}

#[mlua::userdata_impl]
impl QtContext {
    #[lua(skip)]
    pub fn new() -> Self {
        Self{}
    }
}