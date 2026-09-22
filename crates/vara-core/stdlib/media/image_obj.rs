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

    #[lua(infallible)]
    pub fn zero_channel(&mut self, channel: usize) {
        for pixel in self.data.chunks_exact_mut(4) {
            pixel[channel] = 0;
        }
    }

    pub fn flip(this : AnyUserData,colour_channel : Option<u8>) -> mlua::Result<AnyUserData>{

        {
            let mut img = this.borrow_mut::<Image>()?;
            
            let width = img.width as usize;
            let height = img.height as usize;

            for y in 0..height {
                for x in 0..width / 2 {
                    let left = (y * width + x) * 4;
                    let right = (y * width + (width - 1 - x)) * 4;

                    match colour_channel {
                        Some(channel) => img.data.swap(left + channel as usize,right + channel as usize),
                        None => {
                            for c in 0..4 {
                                img.data.swap(left + c,right + c);
                            }
                        }
                    }
                }   
            }
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