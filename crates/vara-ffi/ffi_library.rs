use libloading::Library;
use mlua::ExternalResult;


#[derive(mlua::UserData)]
pub struct ForeignLibrary {
    #[lua(skip)]
    lib: Library,
    #[lua(name="Defined")]
    defined: bool,
}

#[mlua::userdata_impl]
impl ForeignLibrary {
    #[lua(skip)]
    pub fn new(path: String) -> mlua::Result<Self>{
        unsafe{
            Ok(Self { 
                lib: Library::new(path).into_lua_err()?,
                defined: false
            })
        }
    }

    fn define(this: mlua::AnyUserData) -> mlua::Result<mlua::AnyUserData>{
        {
            let foreign_library = this.borrow_mut::<ForeignLibrary>()?;
        }

        Ok(this)
    }
}