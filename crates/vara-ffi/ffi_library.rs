use libloading::Library;
use mlua::ExternalResult;


#[derive(mlua::UserData)]
pub struct ForiegnLibrary {
    #[lua(skip)]
    lib: Library
}

#[mlua::userdata_impl]
impl ForiegnLibrary {
    #[lua(skip)]
    pub fn new(path: String) -> mlua::Result<Self>{
        unsafe{
            Ok(Self { 
                lib: Library::new(path).into_lua_err()? 
            })
        }
    }
}