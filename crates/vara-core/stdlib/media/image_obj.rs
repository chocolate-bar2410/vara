use mlua::{AnyUserData, ExternalError, IntoLua};

use crate::stdlib::media::image_format::image_format_fromstring;

#[derive(Clone,mlua::UserData)]
pub struct Image{
    data : Vec<u8>,
    width : u32,
    height : u32,
}

#[mlua::userdata_impl]
impl Image {
    #[lua(skip)]
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

            if img.data.len() as u32 != width * height * 4 {
                return Err("image dimension does not properly match size params".into_lua_err())
            }

            img.width = width;
            img.height = height;
        }

        Ok(this)
    }

    pub fn flop(this : AnyUserData,colour_channel : Option<u8>) -> mlua::Result<AnyUserData>{

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

    pub fn flip(this : AnyUserData,colour_channel : Option<u8>) -> mlua::Result<AnyUserData>{

        {
            let mut img = this.borrow_mut::<Image>()?;
            
            let width = img.width as usize;
            let height = img.height as usize;

            for y in 0..height/ 2 {
                for x in 0..width  {
                    let top = (y * width + x) * 4;
                    let bottom = ((height - 1 - y) * width + x) * 4;

                    match colour_channel {
                        Some(channel) => img.data.swap(top + channel as usize,bottom + channel as usize),
                        None => {
                            for c in 0..4 {
                                img.data.swap(top + c,bottom + c);
                            }
                        }
                    }
                }   
            }
        }

        Ok(this)
    }

    pub fn greyscale(this : AnyUserData) -> mlua::Result<AnyUserData>{

        {
            let mut img = this.borrow_mut::<Image>()?;
            
            for pixel in img.data.chunks_exact_mut(4) {
                let grey = ((0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32)) as u8;
                pixel[0] = grey;
                pixel[1] = grey;
                pixel[2] = grey;
            }
        }

        Ok(this)
    }

    pub fn convolve(this : AnyUserData,width : usize,height : usize,lua_kernel : mlua::Table) -> mlua::Result<AnyUserData>{
        if lua_kernel.len()? as usize != width * height {
            return Err("kernel does not match kernel width/height parameters provided".into_lua_err())
        }

        let mut kernel : Vec<f64> = Vec::new();
        for value in lua_kernel.sequence_values::<f64>(){
            kernel.push(value?);
        }

        {
            let mut img = this.borrow_mut::<Image>()?;

            let mut result : Vec<u8> = vec![0u8;img.data.len()];
            let half_kernel_width = (width / 2) as isize;
            let half_kernel_height = (height / 2) as isize;

            for y in 0..img.height - 1 {
                for x in 0..img.width - 1 {
                    let output_pixel_index = (y * img.width + x) * 4;

                    for colour_channel in 0..3 {
                        let mut weighted_sum = 0.0f64;

                        for kernel_y in 0..height as isize {
                            for kernel_x in 0..width as isize {

                                let sampled_x : isize= (x as isize + kernel_x as isize - half_kernel_width).clamp(0, (img.width - 1) as isize);
                                let sampled_y : isize = (y as isize + kernel_y as isize - half_kernel_height).clamp(0, (img.height - 1) as isize);

                                let input_pixel_index = (sampled_y * img.width as isize + sampled_x) * 4 + colour_channel;
                                let kernel_weight = kernel[(kernel_y as usize * width) + kernel_x as usize];

                                weighted_sum += img.data[input_pixel_index as usize] as f64 * kernel_weight;
                            }
                        }

                        result[(output_pixel_index + colour_channel as u32) as usize] = weighted_sum.clamp(0.0f64,255.0f64) as u8;
                    }

                    result[(output_pixel_index + 3) as usize] = img.data[(output_pixel_index + 3) as usize];
                }
            }

            img.data = result;
        }

        Ok(this)
    }

    #[lua(name = "clone")]
    pub fn clone_fn(&self) -> mlua::Result<Self>{
        Ok(self.clone())
    }

    #[lua(name="gradientMagnitude")]
    pub fn gradient_magnitude(&self,other : &Image) -> mlua::Result<Self>{
        let mut result = self.clone();

        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {

                let index = ((y * self.width + x) * 4) as usize;

                for cc in 0..3 {
                    let value_a = self.data[index + cc] as f64;
                    let value_b = other.data[index + cc] as f64;

                    let magnitude = value_a.hypot(value_b) as u8;

                    result.data[index + cc] = magnitude;
                }
                
                result.data[index + 3] = 255;
            }
        }

        Ok(result)
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

    #[lua(getter,name = "data")]
    pub fn get_data(&self,context: &mlua::Lua) -> mlua::Result<mlua::Buffer>{

        Ok(context.create_buffer(&self.data)?)
    }

    #[lua(getter,name = "width")]
    pub fn get_width(&self,context: &mlua::Lua) -> mlua::Result<mlua::Value>{

        Ok(self.width.into_lua(&context)?)
    }

    #[lua(getter,name = "height")]
    pub fn get_height(&self,context: &mlua::Lua) -> mlua::Result<mlua::Value>{

        Ok(self.height.into_lua(&context)?)
    }
}