use crate::image_obj::Image;

const RGBA_CHANNELS : usize = 4;

pub fn reinhard(image: &mut Image,exposure: f32){

    for pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){
        for cc in 0..3 {
            let x = pixel[cc] * 2.0f32.powf(exposure);
            pixel[cc] = x / (x + 1.0);
        }
    }
}


pub fn aces(image: &mut Image,config: mlua::Table){
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;

    let exposure = config.get::<f32>("Exposure").unwrap_or(0.0); 

    for pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){

        for cc in 0..3 {

            let x = pixel[cc] * 2.0f32.powf(exposure);

            pixel[cc] = (x * (a * x + b)) / (x * (c * x + d) + e); 
        }
    }
}

pub fn hable(image: &mut Image,config: mlua::Table){
    let a = config.get::<f32>("ShoulderStrength").unwrap_or(0.15);
    let b = config.get::<f32>("LinearStrength").unwrap_or(0.5);
    let c = config.get::<f32>("LinearAngle").unwrap_or(0.1);
    let d = config.get::<f32>("ToeStrength").unwrap_or(0.2);
    let e = config.get::<f32>("ToeNumerator").unwrap_or(0.02);
    let f = config.get::<f32>("ToeDenominator").unwrap_or(0.30);

    let exposure = config.get::<f32>("Exposure").unwrap_or(0.0); 
    let white_x = config.get::<f32>("WhitePoint").unwrap_or(11.2);

    let white_point = 1.0 / ((white_x * (a * white_x + c * b) + d * e) / (white_x * (a * white_x + b) + d * f) - (e/f));

    for pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){

        for cc in 0..3 {
            let x = pixel[cc] * 2.0f32.powf(exposure);

            pixel[cc] = white_point * ((x * (a * x + c * b) + d * e) / (x * (a * x + b) + d * f) - (e/f));
        }
    }
}

fn smoothstep(a: f32,b: f32,x: f32) -> f32{
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);

    t * t * (3.0 - 2.0 * t)
}

pub fn uchimura(image: &mut Image,config: mlua::Table){
    let a = config.get::<f32>("Contrast").unwrap_or(1.0);
    let m = config.get::<f32>("LinearSection").unwrap_or(0.22);
    let l = config.get::<f32>("LinearLength").unwrap_or(0.4);
    let c = config.get::<f32>("Toe").unwrap_or(1.33);
    let b = config.get::<f32>("Pedestal").unwrap_or(0.0);

    let exposure = config.get::<f32>("Exposure").unwrap_or(0.0); 
    let p = config.get::<f32>("WhitePoint").unwrap_or(1.0);

    for pixel in image.data.chunks_exact_mut(RGBA_CHANNELS){
        for cc in 0..3 {

            let x = pixel[cc] * 2.0f32.powf(exposure);

            let l0 = ((p - m) * l) / a;

            let s0 = m + l0;
            let s1 = m + a * l0;
            let c2 = (a * p) / (p - s1);
            let cp = -c2/p;

            let w0 = 1.0 - smoothstep(0.0,m,x);
            let w2 = if x < s0 {0.0} else {1.0};
            let w1 = 1.0 - w0 - w2;

            let toe = m * (x/m).powf(c) + b;
            let shoulder = p - (p - s1) * (cp * (x - s0)).exp();
            let linear = m + a * (x - m);

            pixel[cc] = toe * w0 + linear * w1 + shoulder * w2;
        }

    }
}