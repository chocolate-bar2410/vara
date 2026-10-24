use mlua::{AnyUserData, ExternalError};

use crate::image_format::image_format_fromstring;
use crate::{image_interpolation as interpolation, image_tonemap};

use vara_colour::colour::Colour4;

const RGBA_CHANNELS : u8 = 4;

#[derive(Clone,mlua::UserData)]
pub struct Image{
    #[lua(skip)]
    pub data : Vec<f32>,
    #[lua(get,name = "Width")]
    pub width : u32,
    #[lua(get,name = "Height")]
    pub height : u32,
    #[lua(name = "Channels")]
    pub colour_channels : u8,
}

#[mlua::userdata_impl]
impl Image {
    #[lua(skip)]
    pub fn new(data : Vec<f32>,width : u32,height : u32,colour_channels : u8) -> Self {
        Self { 
            data: data,
            width: width,
            height: height,
            colour_channels: colour_channels
        }
    }

    #[lua(name = "Dimensions")]
    pub fn dimensions(this : AnyUserData,width : u32,height : u32) -> mlua::Result<AnyUserData>{

        {
            let mut img = this.borrow_mut::<Image>()?;

            if img.data.len() as u32 != width * height * RGBA_CHANNELS as u32 {
                return Err("image dimension does not properly match size params".into_lua_err())
            }

            img.width = width;
            img.height = height;
        }

        Ok(this)
    }

    #[lua(name = "Flop")]
    pub fn flop(this : AnyUserData,colour_channel : Option<u8>) -> mlua::Result<AnyUserData>{
        
        {
            let mut img = this.borrow_mut::<Image>()?;

            if let Some(channel) = colour_channel{
                if channel >= RGBA_CHANNELS{
                    return Err("colour channel provided doesn't exist".into_lua_err());
                }
            }

            let width = img.width as usize;
            let height = img.height as usize;

            for y in 0..height {
                for x in 0..width / 2 {
                    let left = (y * width + x) * RGBA_CHANNELS as usize;
                    let right = (y * width + (width - 1 - x)) * RGBA_CHANNELS as usize;

                    match colour_channel {
                        Some(channel) => img.data.swap(left + channel as usize,right + channel as usize),
                        None => {
                            for c in 0..RGBA_CHANNELS as usize {
                                img.data.swap(left + c,right + c);
                            }
                        }
                    }
                }   
            }
        }

        Ok(this)
    }

    #[lua(name = "Flip")]
    pub fn flip(this : AnyUserData,colour_channel : Option<u8>) -> mlua::Result<AnyUserData>{

        {
            let mut img = this.borrow_mut::<Image>()?;
            
            if let Some(channel) = colour_channel{
                if channel >= RGBA_CHANNELS{
                    return Err("colour channel provided doesn't exist".into_lua_err());
                }
            }

            let width = img.width as usize;
            let height = img.height as usize;

            for y in 0..height/ 2 {
                for x in 0..width  {
                    let top = (y * width + x) * RGBA_CHANNELS as usize;
                    let bottom = ((height - 1 - y) * width + x) * RGBA_CHANNELS as usize;

                    match colour_channel {
                        Some(channel) => img.data.swap(top + channel as usize,bottom + channel as usize),
                        None => {
                            for c in 0..RGBA_CHANNELS as usize {
                                img.data.swap(top + c,bottom + c);
                            }
                        }
                    }
                }   
            }
        }

        Ok(this)
    }

    #[lua(name = "Greyscale")]
    pub fn greyscale(this : AnyUserData) -> mlua::Result<AnyUserData>{

        {
            let mut img = this.borrow_mut::<Image>()?;
            
            for pixel in img.data.chunks_exact_mut(RGBA_CHANNELS as usize) {
                let grey = 0.2126 * pixel[0] + 0.7152 * pixel[1] + 0.0722 * pixel[2];
                pixel[0] = grey;
                pixel[1] = grey;
                pixel[2] = grey;
            }
        }

        Ok(this)
    }

    #[lua(name = "Convolve")]
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

            let mut result : Vec<f32> = vec![0f32;img.data.len()];
            let half_kernel_width = (width / 2) as isize;
            let half_kernel_height = (height / 2) as isize;

            for y in 0..img.height {
                for x in 0..img.width {
                    let output_pixel_index = (y * img.width + x) * RGBA_CHANNELS as u32;

                    for colour_channel in 0..3 {
                        let mut weighted_sum = 0.0f32;

                        for kernel_y in 0..height as isize {
                            for kernel_x in 0..width as isize {

                                let sampled_x : isize= (x as isize + kernel_x as isize - half_kernel_width).clamp(0, (img.width - 1) as isize);
                                let sampled_y : isize = (y as isize + kernel_y as isize - half_kernel_height).clamp(0, (img.height - 1) as isize);

                                let input_pixel_index = (sampled_y * img.width as isize + sampled_x) * RGBA_CHANNELS as isize + colour_channel;
                                let kernel_weight = kernel[(kernel_y as usize * width) + kernel_x as usize];

                                weighted_sum += img.data[input_pixel_index as usize] * kernel_weight as f32;
                            }
                        }

                        result[(output_pixel_index + colour_channel as u32) as usize] = weighted_sum;
                    }

                    result[(output_pixel_index + 3) as usize] = img.data[(output_pixel_index + 3) as usize];
                }
            }

            img.data = result;
        }

        Ok(this)
    }

    #[lua(name = "Clone")]
    pub fn clone_fn(&self) -> mlua::Result<Self>{
        Ok(self.clone())
    }

    #[lua(name="GradientMagnitude")]
    pub fn gradient_magnitude(&self,other : &Image) -> mlua::Result<Self>{
        let mut result = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {

                let index = ((y * self.width + x) * RGBA_CHANNELS as u32) as usize;

                for cc in 0..3 {
                    let value_a = self.data[index + cc];
                    let value_b = other.data[index + cc];

                    let magnitude = value_a.hypot(value_b);

                    result.data[index + cc] = magnitude;
                }
                
                result.data[index + 3] = 1.0;
            }
        }

        Ok(result)
    }

    #[lua(name="Resize")]
    pub fn resize(this : AnyUserData,width : u32,height : u32,filter : Option<&str>)  -> mlua::Result<AnyUserData>{
        {
            let mut img = this.borrow_mut::<Image>()?;
            let result = match filter{
                Some(filter_str) => match filter_str.to_ascii_uppercase().trim(){
                    "NEAREST"      => interpolation::nearest_neighbour(&img, width, height)?,
                    "LANCZOS3" => interpolation::lanczos3(&img, width, height)?,
                    "BILINEAR" | _ => interpolation::bilinear(&img, width, height)?,
                },
                None => interpolation::bilinear(&img, width, height)?,
            };

            img.data = result;
            img.width = width;
            img.height = height;
        }
        Ok(this)
    }

    #[lua(name="GetPixel")]
    pub fn get_pixel(&self,x: i64,y: i64) -> mlua::Result<Colour4>{

        if x < 0 || y < 0 || x as u32 > self.width  || y as u32 > self.height  {
            return Err(format!("position {} {} is out of bounds",x,y).into_lua_err());
        }

        let index = (y as u32 * self.width + x as u32) as usize * RGBA_CHANNELS as usize;

        let red = self.data.get(index).ok_or("could not get red channel".into_lua_err())?;
        let green = self.data.get(index + 1).ok_or("could not get green channel".into_lua_err())?;
        let blue = self.data.get(index + 2).ok_or("could not get blue channel".into_lua_err())?;
        let alpha = self.data.get(index + 3).ok_or("could not get alpha channel".into_lua_err())?;

        Ok(Colour4::new(*red, *green, *blue, *alpha))
    }

    #[lua(name="SetPixel")]
    pub fn set_pixel(&mut self,x: i64,y: i64,data: mlua::AnyUserData) -> mlua::Result<()>{

        let colour = data.borrow::<Colour4>()?;

        if x < 0 || y < 0 || x as u32 >= self.width || y as u32 >= self.height  {
            return Err(format!("position {} {} is out of bounds",x,y).into_lua_err());
        }

        let index = (y as u32 * self.width + x as u32) as usize * RGBA_CHANNELS as usize;

        *self.data.get_mut(index).ok_or("could not get red channel".into_lua_err())? = colour.red;
        *self.data.get_mut(index + 1).ok_or("could not get green channel".into_lua_err())? = colour.green;
        *self.data.get_mut(index + 2).ok_or("could not get blue channel".into_lua_err())? = colour.blue;
        *self.data.get_mut(index + 3).ok_or("could not get alpha channel".into_lua_err())? = colour.alpha;

        Ok(())
    }

    #[lua(name="HdrTonemap")]
    pub fn hdr_tonemap(this : AnyUserData,config : Option<mlua::Table>) -> mlua::Result<AnyUserData>{
        {
            let mut image = this.borrow_mut::<Image>()?;

            if config == None {
                image_tonemap::reinhard(&mut image,0.0);
            } else {
                let Some(config) = config else {unreachable!()};

                let filter = config.get::<String>("Filter")?;

                match filter.to_ascii_uppercase().trim() {
                    "ACES" => image_tonemap::aces(&mut image,config),
                    "HABLE" => image_tonemap::hable(&mut image,config),
                    "UCHIMURA" => image_tonemap::uchimura(&mut image,config),
                    "REINHARD" | _ => {
                        image_tonemap::reinhard(&mut image,config.get::<f32>("Exposure").unwrap_or(0.0))
                    },
                }
            }

        }



        Ok(this)
    } 

    #[lua(skip)]
    fn encode_hdr(&self,context: &mlua::Lua,format : image::ImageFormat) -> mlua::Result<mlua::Buffer> {

        if self.colour_channels < 3 {
            return Err("cannot export hdr image with less than 3 channels".into_lua_err())
        }

        let mut bytes = Vec::new();

        let mut data : Vec<f32> = Vec::with_capacity(self.width as usize * self.height as usize * self.colour_channels as usize);

        for pixel in self.data.chunks_exact(4){
            let [red,green,blue,alpha] = [pixel[0],pixel[1],pixel[2],pixel[3]];

            match self.colour_channels {
                3 => data.extend([red,green,blue]),
                4 => data.extend([red,green,blue,alpha]),
                _ => unreachable!()
            }
        }



        match self.colour_channels {
            3 => {
                let image_buffer = image::ImageBuffer::<image::Rgb<f32>,Vec<f32>>::from_raw(self.width,self.height,data)
                    .ok_or(format!("could not encode image to {format:?}"))
                    .map_err(|err| err.into_lua_err())?;

                image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), format).map_err(|err| err.into_lua_err())?;
            },

            4 => {
                let image_buffer = image::ImageBuffer::<image::Rgba<f32>,Vec<f32>>::from_raw(self.width,self.height,data)
                    .ok_or(format!("could not encode image to {format:?}"))
                    .map_err(|err| err.into_lua_err())?;

                image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), format).map_err(|err| err.into_lua_err())?;
            }

            _ => unreachable!()
        };

        context.create_buffer(bytes)
    }

    #[lua(name="Encode")]
    pub fn encode(&self,context : &mlua::Lua,format : String) -> mlua::Result<mlua::Buffer>{

        if !(1..=4).contains(&self.colour_channels) {
            return Err("cannot export image with unsupported channel count".into_lua_err())
        }

        let img_format = image_format_fromstring(&format)?;

        if img_format == image::ImageFormat::Hdr || img_format == image::ImageFormat::OpenExr {
            return self.encode_hdr(context, img_format);
        }

        let mut bytes = Vec::new();

        let mut data : Vec<u8> = Vec::with_capacity(self.width as usize * self.height as usize * self.colour_channels as usize);

        for pixel in self.data.chunks_exact(4){
            let [red,green,blue,alpha] = [pixel[0],pixel[1],pixel[2],pixel[3]];

            let to_srgb = |value: f32| -> f32 {
                if value <= 0.0031308 {value * 12.92} else {value.powf(1.0/2.4) * 1.055 - 0.055}
            };

            let to_u8 = |value: f32| -> u8 {
                (value.clamp(0.0, 1.0) * 255.0).round() as u8
            };

            match self.colour_channels {
                1 => data.push(to_u8(red)),
                2 => data.extend([to_u8(to_srgb(red)),to_u8(alpha)]),
                3 => data.extend([to_u8(to_srgb(red)),to_u8(to_srgb(green)),to_u8(to_srgb(blue))]),
                4 => data.extend([to_u8(to_srgb(red)),to_u8(to_srgb(green)),to_u8(to_srgb(blue)),to_u8(alpha)]),
                _ => unreachable!()
            }

        }


        match self.colour_channels {

            1 => {
                let image_buffer = image::ImageBuffer::<image::Luma<u8>,Vec<u8>>::from_raw(self.width,self.height,data)
                    .ok_or(format!("could not encode image to {format}"))
                    .map_err(|err| err.into_lua_err())?;

                image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), img_format).map_err(|err| err.into_lua_err())?;
            },

            2 => {
                let image_buffer = image::ImageBuffer::<image::LumaA<u8>,Vec<u8>>::from_raw(self.width,self.height,data)
                    .ok_or(format!("could not encode image to {format}"))
                    .map_err(|err| err.into_lua_err())?;

                image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), img_format).map_err(|err| err.into_lua_err())?;
            },

            3 => {
                let image_buffer = image::ImageBuffer::<image::Rgb<u8>,Vec<u8>>::from_raw(self.width,self.height,data)
                    .ok_or(format!("could not encode image to {format}"))
                    .map_err(|err| err.into_lua_err())?;

                image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), img_format).map_err(|err| err.into_lua_err())?;
            },

            4 => {
                let image_buffer = image::ImageBuffer::<image::Rgba<u8>,Vec<u8>>::from_raw(self.width,self.height,data)
                    .ok_or(format!("could not encode image to {format}"))
                    .map_err(|err| err.into_lua_err())?;

                image_buffer.write_to(&mut std::io::Cursor::new(&mut bytes), img_format).map_err(|err| err.into_lua_err())?;
            }

            _ => unreachable!()
        };


        
        context.create_buffer(bytes)
    }

    #[lua(meta,infallible,name = "__tostring")]
    pub fn tostring(&self) -> mlua::Result<String>{
        Ok(format!("image {}x{}",self.width,self.height))
    }

    #[lua(getter,name = "Data")]
    pub fn get_data(&self,context: &mlua::Lua) -> mlua::Result<mlua::Buffer>{

        let bytes: Vec<u8> = self.data.iter().flat_map(|&num| num.to_le_bytes()).collect();

        context.create_buffer(bytes)
    }
}