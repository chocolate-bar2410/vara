use std::{io::Cursor, ops::Div};
use mlua::ExternalError;
use mlua::LuaSerdeExt;

use serde::Deserialize;

use crate::audio_interpolation;

#[derive(mlua::UserData,Clone)]
pub struct Audio{
    #[lua(get,name="Data")]
    pub data: Vec<Vec<f32>>,
    #[lua(get,name="SampleRate")]
    pub sample_rate: u32,
    #[lua(get,name="Frames")]
    pub frames: u64,
    #[lua(get,name="Duration")]
    pub duration: f64,
    #[lua(get,name="Channels")]
    pub channels: u8,
    #[lua(skip)]
    max_amplitude: f32,
}

#[derive(Deserialize,Debug,PartialEq)]
struct CompressionConfig{
    #[serde(default,rename = "Input")]
    input_gain : f32,
    #[serde(default,rename = "Output")]
    output_gain : f32,
    #[serde(default,rename = "Threshold")]
    threshold : f32,
    #[serde(default,rename = "Ratio")]
    ratio : f32,
    #[serde(default,rename = "Attack")]
    attack : f32,
    #[serde(default,rename = "Release")]
    release : f32,
    #[serde(default,rename = "Knee")]
    knee : f32,
}

#[mlua::userdata_impl]
impl Audio {
    // constructor / utility methods
    #[lua(skip)]
    pub fn new(data: Vec<Vec<f32>>,sample_rate: u32,frames: u64) -> mlua::Result<Self>{
        let channels = data.len() as u8;
        let mut audio = Self { 
            data: data,
            sample_rate: sample_rate,
            frames: frames,
            duration : frames as f64 / sample_rate as f64,
            channels : channels,
            max_amplitude : 0.0f32
        };

        audio.compute_max_amplitude();
        Ok(audio)
    }

    #[lua(skip)]
    fn compute_max_amplitude(&mut self){
        self.max_amplitude = self.data.iter().flatten().fold(-f32::INFINITY,|max,&value| max.max(value.abs()));
    }

    #[lua(skip)]
    fn dbfs_to_pcm(&self,decibels: f32) -> f32 {
        10.0f32.powf(decibels / 20.0f32)
    }

    // mixing methods

    #[lua(name="Amplify")]
    pub fn amplify(this : mlua::AnyUserData,decibels: f32) ->mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let gain_pcm = audio.dbfs_to_pcm(decibels);

            for channel in &mut audio.data {
                for value in channel {
                    *value *= gain_pcm;
                }
            }

            audio.compute_max_amplitude();
        }

        Ok(this)
    }

    #[lua(name="Lowpass")]
    pub fn lowpass(this : mlua::AnyUserData,cutoff_hz : f32,q_factor : Option<f32>)  -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let q_factor = match q_factor {
                Some(q) => q,
                None => std::f32::consts::FRAC_1_SQRT_2,
            };

            let omega = 2.0 * (std::f32::consts::TAU * cutoff_hz) / audio.sample_rate as f32;
            let intermediate = omega.div(2.0).tan();

            let norm_factor = 1.0/(1.0 + (intermediate / q_factor) + intermediate.powf(2.0));

            let a0 = intermediate.powf(2.0) * norm_factor;
            let a1 = 2.0 * a0;
            let a2 = a0;
            
            let b0 = 2.0 * (intermediate.powf(2.0) - 1.0) * norm_factor;
            let b1 = (1.0 - (intermediate / q_factor) + intermediate.powf(2.0)) * norm_factor;

            audio.biquad_filter(a0,a1,a2,b0,b1);
        }
        Ok(this)
    }

    #[lua(name="Highpass")]
    pub fn highpass(this : mlua::AnyUserData,cutoff_hz : f32,q_factor : Option<f32>)  -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let q_factor = match q_factor {
                Some(q) => q,
                None => std::f32::consts::FRAC_1_SQRT_2,
            };

            let omega = (std::f32::consts::TAU * cutoff_hz) / audio.sample_rate as f32;
            let alpha = omega.sin().div(2.0 * q_factor);


            let a0 = 1.0 + alpha;
            let a1 = (-2.0 * omega.cos()).div(a0);
            let a2 = (1.0 - alpha).div(a0);
            
            let b0 = (1.0 + omega.cos()).div(2.0).div(a0);
            let b1 = -(1.0 + omega.cos()).div(a0);
            let b2 = b0;

            audio.biquad_filter(b0,b1,b2,a1,a2);
        }
        Ok(this)
    }

    #[lua(name="Notch")]
    pub fn notch(this : mlua::AnyUserData,center_hz : f32,q_factor : Option<f32>)  -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let q_factor = match q_factor {
                Some(q) => q,
                None => std::f32::consts::FRAC_1_SQRT_2,
            };

            let omega = (std::f32::consts::TAU * center_hz) / audio.sample_rate as f32;
            let alpha = omega.sin().div(2.0 * q_factor);

            let a0 = 1.0 + alpha;
            let a1 = (-2.0 * omega.cos()).div(a0);
            let a2 = (1.0 - alpha).div(a0);

            let b0 = 1.0 / a0;
            let b1 = (-2.0 * omega.cos()).div(a0);
            let b2 = 1.0 / a0;

            audio.biquad_filter(b0,b1,b2,a1,a2);
        }
        Ok(this)
    }

    #[lua(skip)]
    pub fn biquad_filter(&mut self,a0: f32,a1: f32,a2: f32,b0: f32,b1: f32){

        for channel_index in 0..self.data.len(){
            let channel = &mut self.data[channel_index];

            let (mut x1, mut x2, mut y1, mut y2) = (0.0f32,0.0f32,0.0f32,0.0f32);

            for sample in channel.iter_mut(){
                let x0 = *sample;
                let y0 = a0 * x0 + a1 * x1 + a2 * x2 - b0 * y1 - b1 * y2;

                x2 = x1;
                x1 = x0;

                y2 = y1;
                y1 = y0;

                *sample = y0;
            }
        }

        self.compute_max_amplitude();
    }

    #[lua(name="Normalise")]
    pub fn normalise(this : mlua::AnyUserData,decibel: Option<f32>) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            if audio.max_amplitude == 0.0 { return Ok(this) }

            let gain = match decibel{
                Some(_decibel) => audio.dbfs_to_pcm(_decibel) / audio.max_amplitude,
                None => 1.0 / audio.max_amplitude,
            };


            for channel in audio.data.iter_mut() {
                for value in channel.iter_mut() {
                    *value *= gain;
                }
            }

            audio.compute_max_amplitude();

        }
        Ok(this)
    }
    
    #[lua(name="HardClip")]
    pub fn hard_limit(this : mlua::AnyUserData,threshold : f32) -> mlua::Result<mlua::AnyUserData>{
         {
            let mut audio = this.borrow_mut::<Audio>()?;
            let threshold_pcm = audio.dbfs_to_pcm(threshold);

            for channel in &mut audio.data {
                for i in 0..channel.len() as usize {
                    channel[i] = channel[i].clamp(-threshold_pcm, threshold_pcm);
                }
            }

            audio.compute_max_amplitude();
         }
        Ok(this)
    }

    #[lua(name="SoftClip")]
    pub fn soft_limit(this : mlua::AnyUserData,threshold : f32) -> mlua::Result<mlua::AnyUserData>{
         {
            let mut audio = this.borrow_mut::<Audio>()?;
            let threshold_pcm = audio.dbfs_to_pcm(threshold).abs();

            for channel in &mut audio.data {
                for sample in channel.iter_mut() {
                    let magnitude = sample.abs();

                    if magnitude <= threshold_pcm {continue;}

                    let u = ((magnitude - threshold_pcm) / (1.0 - threshold_pcm)).min(1.0);
                    let smooth = u * u * u * (u * (u * 6.0 - 15.0) + 10.0);

                    *sample = (threshold_pcm + (1.0 - threshold_pcm) * smooth).copysign(*sample);
                }
            }

            audio.compute_max_amplitude();
         }
        Ok(this)
    }

    #[lua(name="Saturate")]
    pub fn saturate(this : mlua::AnyUserData,drive_db : f32,mode : Option<&str>) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let drive = audio.dbfs_to_pcm(drive_db);
            let norm = std::f32::consts::FRAC_PI_2 * drive.atan();

            for channel in &mut audio.data {
                for sample in channel.iter_mut() {
                    let x = *sample * drive;

                    *sample = match mode { 
                        Some(mode_string) => match mode_string{
                            "ARCTAN"   => (std::f32::consts::FRAC_PI_2 * x.atan()) / norm,
                            "TANH" | _ => x.tanh() / drive.tanh(),
                        },
                        None => x.tanh() / drive.tanh(),
                    };
                }
            }
        }
        Ok(this)
    }

    #[lua(name="Compress")]
    pub fn compress(context : &mlua::Lua,this : mlua::AnyUserData,config_table : mlua::Value) -> mlua::Result<mlua::AnyUserData>{
        
        let config : CompressionConfig = context.from_value(config_table)?;

        let threshold = config.threshold;
        let knee = config.knee;
        let ratio = config.ratio;

        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let attack_coeff = (-1.0 / (config.attack * audio.sample_rate as f32)).exp();
            let release_coeff = (-1.0 / (config.release * audio.sample_rate as f32)).exp();

            let input_pcm = audio.dbfs_to_pcm(config.input_gain);
            let output_pcm = audio.dbfs_to_pcm(config.output_gain);

            let mut envelope = 0.0f32;

            for frame in 0..audio.frames as usize {
                let level = audio.data
                    .iter()
                    .map(|channel| (channel[frame] * input_pcm).abs())
                    .fold(0.0,f32::max);

                if level > envelope {
                    envelope += (level - envelope) * (1.0 - attack_coeff);
                } else {
                    envelope += (level - envelope) * (1.0 - release_coeff);
                }

                // compressor algorithm works better with decibels for some reason
                let envelope_db = 20.0 * envelope.max(1e-20).log10();

                let reduction_db = if knee <= 0.0 {
                    if envelope_db <= threshold {0.0} else {threshold + ((envelope_db - threshold) / ratio) - envelope_db}
                } else {
                    let lower = threshold - knee * 0.5;
                    let upper = threshold + knee * 0.5;

                    if envelope_db < lower {
                        0.0
                    } else if envelope_db > upper {
                        (threshold + (envelope_db - threshold) / ratio) - envelope_db
                    } else {
                        let x = envelope_db - lower;
                        let compressed = x * x / (2.0 * knee);

                        -(1.0 - 1.0 / ratio) * compressed
                    }
                };

                let gain = 10.0f32.powf(reduction_db / 20.0);

                for channel in &mut audio.data {
                    let sample = channel[frame] * input_pcm;
                    channel[frame] = sample * gain * output_pcm;
                }
            }

            audio.compute_max_amplitude();
        }

        Ok(this)
    }
    // fades

    #[lua(name="FadeIn")]
    pub fn fade_in(this : mlua::AnyUserData,duration: f32) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;
            let fade_samples = ((duration * audio.sample_rate as f32) as usize).min(audio.frames as usize);

            let safe_count = fade_samples.saturating_sub(1);
            let divisor = (safe_count as f32).max(1.0);

            let start = 0.0;
            let end = 1.0;

            for i in 0..fade_samples {
                let t = i as f32 / divisor;
                let gain = start + (end - start) * t;

                for channel in &mut audio.data{
                    channel[i] *= gain;
                }
            }

            audio.compute_max_amplitude();
        }
        Ok(this)
    }
    
    #[lua(name="FadeOut")]
    pub fn fade_out(this : mlua::AnyUserData,duration: f32) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let min_len = audio.data.iter().map(|channel| channel.len()).min().unwrap_or(0);
            let fade_samples = ((duration * audio.sample_rate as f32) as usize).min(audio.frames as usize).min(min_len);

            if fade_samples <= 1 {return Ok(this)}

            let start_sample = min_len.saturating_sub(fade_samples);
            let divisor = (fade_samples - 1) as f32;

            let start = 1.0;
            let end = 0.0;

            for i in 0..fade_samples {
                let t = i as f32 / divisor;
                let gain = start + (end - start) * t;
                let sample_index = start_sample + i;

                for channel in &mut audio.data{
                    channel[sample_index] *= gain;
                }
            }

            audio.compute_max_amplitude();
        }
        Ok(this)
    }

    #[lua(name="Trim")]
    pub fn trim(this : mlua::AnyUserData,start: f32,end: Option<f32>) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            let start_frame = (start * audio.sample_rate as f32) as usize;
            let end_frame = end.map(|e| (e * audio.sample_rate as f32) as usize).unwrap_or(audio.frames as usize);

            for channel in &mut audio.data{
                channel.truncate(end_frame.min(channel.len()));
                channel.drain(0..start_frame.min(channel.len()));
            }

            audio.frames = audio.data[0].len() as u64;
            audio.duration = audio.frames as f64 / audio.sample_rate as f64;
    
            audio.compute_max_amplitude();
        }

        Ok(this)
    }

    // multi audio methods

    #[lua(name="Append")]
    pub fn append(this : mlua::AnyUserData,
        other : mlua::AnyUserData,
        start : Option<f32>,
        stop : Option<f32>,
        crossfade : Option<f32>
    ) -> mlua::Result<mlua::AnyUserData>{
        {
            let other_audio = {
                let other_ref = other.borrow::<Audio>()?;

                other_ref.clone()
            };
            let mut audio = this.borrow_mut::<Audio>()?;
            
            if audio.sample_rate != other_audio.sample_rate {
                return Err("cannot append audio with an audio of a different sample rate".into_lua_err())
            }

            if audio.channels != other_audio.channels {
                return Err("cannot append audio with an audio with a different ammount of channels".into_lua_err())
            }

            let mut start_frame = start.map(|seconds| (seconds * other_audio.sample_rate as f32) as usize).unwrap_or(0 as usize);
            let mut stop_frame = stop.map(|seconds| (seconds * other_audio.sample_rate as f32) as usize).unwrap_or(other_audio.frames as usize);

            start_frame = start_frame.clamp(0, stop_frame);
            stop_frame = stop_frame.clamp(start_frame, other_audio.frames as usize);

            

            if let Some(crossfade_seconds) = crossfade && crossfade_seconds > 0.0 {
                let append_length = stop_frame.saturating_sub(start_frame);

                let crossfade_frames = ((crossfade_seconds * other_audio.sample_rate as f32) as usize)
                    .min(audio.frames as usize)
                    .min(append_length);

                for channel_index in 0..audio.channels as usize {

                    let channel = &mut audio.data[channel_index];
                    let other_channel = &other_audio.data[channel_index];

                    let crossfade_start = channel.len() - crossfade_frames;

                    for i in 0..crossfade_frames{
                        let t = i as f32 / crossfade_frames.saturating_sub(1).max(1) as f32;

                        let this_sample = channel[crossfade_start + i];
                        let other_sample = other_channel[start_frame + i];
                        channel[crossfade_start + i] = this_sample * (1.0 - t).sqrt() + other_sample * t.sqrt();
                    }

                    for index in (start_frame + crossfade_frames)..stop_frame{
                        channel.push(other_channel[index]);
                    }
                }

            } else {
                 for channel_index in 0..audio.channels as usize {
                    let channel = &mut audio.data[channel_index];

                    for index in start_frame..stop_frame{
                        channel.push(other_audio.data[channel_index][index]);
                    }
                }
            }

            audio.frames = audio.data[0].len() as u64;
            audio.duration = audio.frames as f64 / audio.sample_rate as f64;

            audio.compute_max_amplitude();
        }

        Ok(this)
    }

    #[lua(name="Overlay")]
    pub fn overlay(this : mlua::AnyUserData,other : mlua::AnyUserData,position : Option<f32>,repeat : Option<bool>) -> mlua::Result<mlua::AnyUserData>{
            {
                let other_audio = {
                    let other_ref = other.borrow::<Audio>()?;

                    other_ref.clone()
                };

                let mut audio = this.borrow_mut::<Audio>()?;

                if audio.sample_rate != other_audio.sample_rate {
                    return Err("cannot overlay audio with an audio of a different sample rate".into_lua_err())
                }

                if audio.channels != other_audio.channels {
                    return Err("cannot overlay audio with an audio with a different ammount of channels".into_lua_err())
                }

                let start_frame = position.map(|seconds| seconds * audio.sample_rate as f32).unwrap_or(0.0) as usize;

                let should_repeat = repeat.unwrap_or(false);

                for channel_index in 0..audio.channels as usize {
                    let channel = &mut audio.data[channel_index];
                    let other_channel = &other_audio.data[channel_index];

                    let available = channel.len().saturating_sub(start_frame);

                    if should_repeat {
                        for index in 0..available{
                            channel[start_frame + index] += other_channel[index % other_channel.len()];
                        }
                    } else {
                        let count = other_channel.len().min(available);

                        for index in 0..count{
                            channel[start_frame + index] += other_channel[index];
                        }
                    }

                    
                }

                audio.compute_max_amplitude();
            }

            Ok(this)
        }

    // misc.

    #[lua(name="Resample")]
    pub fn resample(this : mlua::AnyUserData,sample_rate : u32,filter : Option<&str>)  -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;

            if audio.sample_rate == sample_rate {return Ok(this)}

            let ratio = audio.sample_rate as f32 / sample_rate as f32;
            let new_frames = (audio.frames as f32 / ratio).round() as usize;

            match filter {
                Some(filter_str) => match filter_str.to_ascii_uppercase().trim() {
                    "LINEAR"      => audio_interpolation::linear_interpolation(&mut audio, new_frames, ratio),
                    "LANCZOS3"    => audio_interpolation::lanczos3_interpolation(&mut audio, new_frames, ratio),
                    "HERMITE" | _ => audio_interpolation::hermite_interpolation(&mut audio, new_frames, ratio),
                }
                | None => audio_interpolation::hermite_interpolation(&mut audio, new_frames, ratio),
            }

            audio.sample_rate = sample_rate;
            audio.frames = new_frames as u64;
            audio.duration = new_frames as f64 / sample_rate as f64;

            audio.compute_max_amplitude();
        }

        Ok(this)
    }

    #[lua(name="Clone")]
    pub fn clone_fn(&self) -> mlua::Result<Self>{
        Ok(self.clone())
    }

    #[lua(name="Reverse")]
    pub fn reverse(this : mlua::AnyUserData) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut _audio = this.borrow_mut::<Audio>()?;

            for channel in &mut _audio.data {
                channel.reverse();
            }
        }
        Ok(this)
    }

    #[lua(name="Encode")]
    pub fn encode(&self,context: &mlua::Lua,format: String) -> mlua::Result<mlua::Buffer>{
        match format.to_ascii_uppercase().trim() {
            "WAV" => {
                let spec = hound::WavSpec{
                    channels : self.channels as u16,
                    sample_rate : self.sample_rate,
                    bits_per_sample : 32,
                    sample_format : hound::SampleFormat::Float,
                };

                let mut cursor = Cursor::new(Vec::new());
                let mut writer = hound::WavWriter::new(&mut cursor,spec).map_err(|err|err.into_lua_err())?;

                for sample_index in 0..self.frames as usize{
                    for channel in &self.data {
                        writer.write_sample(channel[sample_index]).map_err(|err|err.into_lua_err())?;
                    }
                }

                writer.finalize().map_err(|err|err.into_lua_err())?;

                context.create_buffer(cursor.into_inner())
            },
            "FLAC" => {
                let bits_per_sample = 24;
                let max_val = ((1u64 << (bits_per_sample - 1)) - 1) as f32;
                let mut enc = rusty_flac::Encoder::new(self.sample_rate,self.channels.into(),bits_per_sample).map_err(|err|err.into_lua_err())?;

                let mut interleaved = Vec::new();

                for sample_index in 0..self.frames as usize{
                    for channel in &self.data {
                        interleaved.push((channel[sample_index].clamp(-1.0, 1.0) * max_val) as i32);
                    }
                }

                enc.push_interleaved(&interleaved).map_err(|err|err.into_lua_err())?;

                context.create_buffer(enc.finish())
            }
            
            _ =>return Err("encoding non .wav files is unimplemetned".into_lua_err())
        }


        
    }

    // metamethods

   #[lua(meta,name = "__tostring")]
    pub fn to_string(&self) -> mlua::Result<String>{
        Ok(format!("audio {}ch {}Hz {} frames",self.channels,self.sample_rate,self.frames))
    }
}