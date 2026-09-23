use crate::stdlib::media::image_obj::Image;

pub fn bilinear(img : &mlua::UserDataRefMut<Image>,width : u32, height : u32) -> mlua::Result<Vec<u8>>{
    let mut result : Vec<u8> = vec![0;(width * height * img.colour_channels as u32) as usize];
    let channels = img.colour_channels as usize;
    
    for y in 0..height{
        for x in 0..width{
            let source_x = x as f32 * (img.width - 1) as f32 / width.max(2).saturating_sub(1) as f32;
            let source_y = y as f32 * (img.height - 1) as f32 / height.max(2).saturating_sub(1) as f32;

            let x0 = source_x.floor() as usize;
            let y0 = source_y.floor() as usize;

            let x1 = (x0 + 1).min(img.width as usize - 1) as usize;
            let y1 = (y0 + 1).min(img.height as usize - 1) as usize;

            let dx = source_x - x0 as f32;
            let dy = source_y - y0 as f32;

            for cc in 0..channels{
                let p00 = img.data[(y0 * img.width as usize + x0) * channels + cc] as f32;
                let p10 = img.data[(y0 * img.width as usize + x1) * channels + cc] as f32;
                let p01 = img.data[(y1 * img.width as usize + x0) * channels + cc] as f32;
                let p11 = img.data[(y1 * img.width as usize + x1) * channels + cc] as f32;

                let value = 
                p00 * (1.0 - dx) * (1.0 - dy) +
                p10 * dx * (1.0 - dy) +
                p01 * (1.0 - dx) * dy +
                p11 * dx * dy;

                result[(y as usize * width as usize + x as usize) * channels + cc] = value.round() as u8;
            }
        }
    }

    Ok(result)
}

pub fn nearest_neighbour(img : &mlua::UserDataRefMut<Image>,width : u32, height : u32) -> mlua::Result<Vec<u8>>{
    let mut result : Vec<u8> = vec![0;(width * height * img.colour_channels as u32) as usize];
    let channels = img.colour_channels as usize;
    
    for y in 0..height{
        for x in 0..width{
            let source_x = x as f32 * (img.width - 1) as f32 / width.max(2).saturating_sub(1) as f32;
            let source_y = y as f32 * (img.height - 1) as f32 / height.max(2).saturating_sub(1) as f32;

            let closest_x = source_x.round() as usize;
            let closest_y = source_y.round() as usize;

            for cc in 0..channels{
                result[(y as usize * width as usize + x as usize) * channels + cc] = img.data[(closest_y * img.width as usize + closest_x) * channels + cc];
            }
        }
    }

    Ok(result)
}