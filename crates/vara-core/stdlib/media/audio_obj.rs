use std::io::Cursor;

use mlua::ExternalError;

#[derive(mlua::UserData,Clone)]
pub struct Audio{
    #[lua(get)]
    data: Vec<Vec<f32>>,
    #[lua(get,name="sampleRate")]
    sample_rate: u32,
    #[lua(get)]
    frames: u64,
    #[lua(get)]
    duration: f64,
    #[lua(get)]
    channels: u8,
    #[lua(skip)]
    max_amplitude: f32,
}

#[mlua::userdata_impl]
impl Audio {
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
    #[lua(name="fadeIn")]
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
    #[lua(name="fadeOut")]
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


    #[lua(name="clone")]
    pub fn clone_fn(&self) -> mlua::Result<Self>{
        Ok(self.clone())
    }

    pub fn reverse(this : mlua::AnyUserData) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut _audio = this.borrow_mut::<Audio>()?;

            for channel in &mut _audio.data {
                channel.reverse();
            }
        }
        Ok(this)
    }

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

   #[lua(meta,name = "__tostring")]
    pub fn to_string(&self) -> mlua::Result<String>{
        Ok(format!("audio {}ch {}Hz {} frames",self.channels,self.sample_rate,self.frames))
    }
}