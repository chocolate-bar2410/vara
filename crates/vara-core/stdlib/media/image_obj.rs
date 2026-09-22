
pub struct Image{
    data : Vec<u8>,
    width : u32,
    height : u32,
}

impl Image {
    pub fn new(data : Vec<u8>,width : u32,height : u32) -> Self {
        Self { 
            data: data,
            width: width,
            height: height 
        }
    }
}

impl mlua::UserData for Image {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_,this,()|{
            Ok(format!("image {}x{}",this.width,this.height))
        });
    }
}