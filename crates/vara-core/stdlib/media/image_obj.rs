use mlua::{AnyUserData, ExternalError};

use crate::stdlib::media::image_format::image_format_fromstring;

#[derive(Clone,mlua::UserData)]
pub struct Image{
    data : Vec<u8>,
    width : u32,
    height : u32,
}

#[mlua::userdata_impl]
impl Image {
    #[lua(infallible)]
    pub fn new(data : Vec<u8>,width : u32,height : u32) -> Self {
        Self { 
            data: data,
            width: width,
            height: height ,
        }
    }

    pub fn dimensions(this : AnyUserData,width : u32,height : u32) -> mlua::Result<AnyUserData>{
        {
            let mut img = this.borrow_mut::<Image>()?;
            img.width = width;
            img.height = height;
        }

        Ok(this)
    }

    pub fn encode(&self,context : &mlua::Lua,format : String) -> mlua::Result<mlua::Buffer>{
        let img_format = image_format_fromstring(&format)?;

        let image_buffer = image::ImageBuffer::<image::Rgba<u8>,Vec<u8>>::from_raw(self.width,self.height,self.data.clone())
        .expect(&format!("unable to encode to {}",format).to_string());

        let mut bytes = Vec::new();

        image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), img_format).map_err(|err| err.into_lua_err())?;

        context.create_buffer(bytes)
    }

    #[lua(meta,infallible,name = "__tostring")]
    pub fn tostring(&self) -> mlua::Result<String>{
        Ok(format!("image {}x{}",self.width,self.height))
    }
}