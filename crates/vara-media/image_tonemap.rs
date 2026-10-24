use crate::image_obj::Image;

const RGBA_CHANNELS : usize = 4;

pub fn reinhard(image: &mut Image,exposure: f32){

    for pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){
        for cc in 0..RGBA_CHANNELS {
            let x = pixel[cc] * 2.0f32.powf(exposure);
            pixel[cc] = x / (x + 1.0);
        }
    }
}


pub fn aces(image: &mut Image,config: mlua::Table){
    let a = config.get::<f32>("A").unwrap_or(2.51);
    let b = config.get::<f32>("B").unwrap_or(0.03);
    let c = config.get::<f32>("C").unwrap_or(2.43);
    let d = config.get::<f32>("D").unwrap_or(0.59);
    let e = config.get::<f32>("E").unwrap_or(0.14);

    let exposure = config.get::<f32>("Exposure").unwrap_or(0.0); 

    for pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){

        for cc in 0..RGBA_CHANNELS {

            let x = pixel[cc] * 2.0f32.powf(exposure);

            pixel[cc] = (x * (a * x + b)) / (x * (c * x + d) + e); 
        }
    }
}

pub fn hable(image: &mut Image,_config: mlua::Table){
    for _pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){
    
    }
}

pub fn agx(image: &mut Image,_config: mlua::Table){
    for _pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){
    
    }
}