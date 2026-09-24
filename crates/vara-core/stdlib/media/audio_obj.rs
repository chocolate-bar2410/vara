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
    duration: u64,
    #[lua(get)]
    channels: u8,
}

#[mlua::userdata_impl]
impl Audio {
    #[lua(skip)]
    pub fn new(data: Vec<Vec<f32>>,sample_rate: u32,frames: u64) -> mlua::Result<Self>{
        let channels = data.len() as u8;
        
        Ok(Self { 
            data: data,
            sample_rate: sample_rate,
            frames: frames,
            duration : frames / sample_rate as u64,
            channels : channels,
        })
    }

    #[lua(skip)]
    fn dbfs_to_pcm(&self,decibels: f32) -> f32 {
        10.0f32.powf(decibels / 20.0f32)
    }

    pub fn amplify(this : mlua::AnyUserData,decibels: f32) ->mlua::Result<mlua::AnyUserData>{
        {
            let mut _audio = this.borrow_mut::<Audio>()?;

            let gain_pcm = _audio.dbfs_to_pcm(decibels);

            for channel in &mut _audio.data {
                for value in channel {
                    *value *= gain_pcm;
                }
            }
        }

        Ok(this)
    }

    pub fn normalise(this : mlua::AnyUserData,decibel: Option<f32>) -> mlua::Result<mlua::AnyUserData>{
        {
            let mut audio = this.borrow_mut::<Audio>()?;
            let max_amplitude = audio.data
                .iter()
                .flatten()
                .fold(-f32::INFINITY,|max,&value| max.max(value.abs())) as f32;

            if max_amplitude == 0.0 { return Ok(this) }

            let gain = match decibel{
                Some(_decibel) => audio.dbfs_to_pcm(_decibel) / max_amplitude,
                None => 1.0 / max_amplitude,
            };


            for channel in audio.data.iter_mut() {
                for value in channel.iter_mut() {
                    *value *= gain;
                }
            }

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