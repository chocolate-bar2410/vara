use crate::stdlib::media::audio_obj::Audio;

use std::f32::consts::PI;

pub fn hermite_interpolation(audio : &mut Audio,new_frames : usize,ratio : f32){

    for channel in &mut audio.data {
        let mut new_channel = vec![0.0f32;new_frames];

        for i in 0..new_frames {
            let exact_position = i as f32 * ratio;
            let index = exact_position.floor() as usize;
            let t = exact_position - index as f32;

            let y0 = channel.get(index.saturating_sub(1)).copied().unwrap_or(0.0);
            let y1 = channel.get(index).copied().unwrap_or(0.0);
            let y2 = channel.get(index + 1).copied().unwrap_or(0.0);
            let y3 = channel.get(index + 2).copied().unwrap_or(0.0);

            let c0 = y1;
            let c1 = 0.5 * (y2 - y0);
            let c2 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
            let c3 = 0.5 * (y3 - y0) + 1.5 * (y1 - y2);
            
            // horners method for polynomial evaluation
            new_channel[i] = ((c3 * t + c2) * t + c1) * t + c0
        }

        *channel = new_channel;
    }
}

pub fn linear_interpolation(audio : &mut Audio,new_frames : usize,ratio : f32){

    for channel in &mut audio.data {
        let mut new_channel = vec![0.0f32;new_frames];

        for i in 0..new_frames {
            let exact_position = i as f32 * ratio;
            let index = exact_position.floor() as usize;
            let t = exact_position - index as f32;

            let y0 = channel.get(index).copied().unwrap_or(0.0);
            let y1 = channel.get(index + 1).copied().unwrap_or(0.0);
            
            new_channel[i] = y0 - (y1 - y0) * t;
        }

        *channel = new_channel;
    }
}

fn sinc(x : f32) -> f32 {
    if x.abs() < 1e-6 { 1.0 } else { (PI * x).sin() / (PI * x) }
}

fn lanczos(a : f32,b : f32) -> f32 {
    let a = a.abs();
    if a < b { sinc(a) * sinc(a/b) } else { 0.0 }
}

pub fn lanczos3_interpolation(audio : &mut Audio,new_frames : usize,ratio : f32){
    for channel in &mut audio.data {
        let mut new_channel = vec![0.0f32;new_frames];

        for i in 0..new_frames {
            let exact_position = i as f32 * ratio;
            let index = exact_position.floor() as usize;
            let t = exact_position - index as f32;

            let mut sum = 0.0f32;
            let mut weighted_sum = 0.0f32;

            for offset in -2..=3 {
                let sample_index = index as isize + offset;
                let distance = t - offset as f32;
                let weight = lanczos(distance, 3.0f32);

                let sample = if (0..channel.len()).contains(&(sample_index as usize)){channel[sample_index as usize]} else {0.0};

                sum += sample * weight;
                weighted_sum += weight;
            }

            new_channel[i] = if weighted_sum.abs() > 1e-6 {sum / weighted_sum} else {0.0}
        }

        *channel = new_channel;
    }
}