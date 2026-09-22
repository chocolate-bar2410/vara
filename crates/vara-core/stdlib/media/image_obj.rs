use mlua::ExternalError;

use crate::stdlib::media::image_format::image_format_fromstring;

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
        methods.add_method("encode", |context : &mlua::Lua,this,format : String|{

            let img_format = image_format_fromstring(&format)?;

            let image_buffer = image::ImageBuffer::<image::Rgba<u8>,Vec<u8>>::from_raw(this.width,this.height,this.data.clone())
            .expect(&format!("unable to encode to {}",format).to_string());

            let mut bytes = Vec::new();

            image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), img_format).map_err(|err| err.into_lua_err())?;

            context.create_buffer(bytes)
        });

        methods.add_meta_method(mlua::MetaMethod::ToString, |_,this,()|{
            Ok(format!("image {}x{}",this.width,this.height))
        });
    }
}