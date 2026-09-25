use crate::stdlib::media::audio_obj::Audio;

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