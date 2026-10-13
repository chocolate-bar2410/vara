use crate::image_obj::Image;

use std::f32::consts::PI;

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

fn sinc(x : f32) -> f32 {
    if x.abs() < 1e-6 { 1.0 } else { (PI * x).sin() / (PI * x) }
}

fn lanczos(a : f32,b : f32) -> f32 {
    let a = a.abs();
    if a < b { sinc(a) * sinc(a/b) } else { 0.0 }
}

pub fn lanczos3(img : &mlua::UserDataRefMut<Image>,width : u32, height : u32) -> mlua::Result<Vec<u8>>{
    let mut temp : Vec<f32> = vec![0.0;(width * img.height * img.colour_channels as u32) as usize];
    let mut result : Vec<u8> = vec![0;(width * height * img.colour_channels as u32) as usize];
    let channels = img.colour_channels as usize;
    
    let temp_row_stride = width as usize * channels;
    let img_row_stride = img.width as usize * channels;

    let x_scale = (img.width - 1) as f32 / width.max(2).saturating_sub(1) as f32;
    let y_scale = (img.height - 1) as f32 / height.max(2).saturating_sub(1) as f32;

    for (_,(out_row,in_row)) in temp
        .chunks_exact_mut(temp_row_stride)
        .zip(img.data.chunks_exact(img_row_stride))
        .enumerate()
    {
        for (x,out_pixel) in out_row.chunks_exact_mut(channels).enumerate() {
            let source_x = x as f32 * x_scale;

            let base_index = source_x.floor() as isize;
            
            let mut sums = [0.0f32;4];
            let mut weighted_sum = 0.0f32;

            for i in -2..=3 {
                let sx = (base_index + i).clamp(0,img.width as isize - 1) as usize;

                let weight = lanczos(source_x - sx as f32, 3.0 as f32);
                weighted_sum += weight;

                let in_pixel = &in_row[sx * channels..(sx + 1) * channels];

                for cc in 0..channels{
                    sums[cc] += weight * in_pixel[cc] as f32
                }
            }

            let inv_weighted_sum = 1.0 / weighted_sum;
            for cc in 0..channels{
                out_pixel[cc] = sums[cc] * inv_weighted_sum;
            }
        }
    }

    for (y,out_row) in result.chunks_exact_mut(temp_row_stride).enumerate(){
        let source_y = y as f32 * y_scale;
        let base_index = source_y.floor() as isize;

        for (x,out_pixel) in out_row.chunks_exact_mut(channels).enumerate() {
            let mut sums = [0.0f32;4];
            let mut weighted_sum = 0.0f32;

            for i in -2..=3 {
                let sy = (base_index + i).clamp(0,img.height as isize - 1) as usize;

                let weight = lanczos(source_y - sy as f32, 3.0 as f32);
                weighted_sum += weight;

                let pixel_offset = (sy * width as usize + x) * channels;
                let temp_pixel = &temp[pixel_offset..pixel_offset + channels];

                for cc in 0..channels{
                    sums[cc] += weight * temp_pixel[cc] as f32
                }
            }

            let inv_weighted_sum = 1.0 / weighted_sum;
            for cc in 0..channels{
                out_pixel[cc] = (sums[cc] * inv_weighted_sum).clamp(0.0, 255.0) as u8;
            }
        }
    }

    Ok(result)
}