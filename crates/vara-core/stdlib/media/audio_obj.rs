use std::io::Cursor;

use mlua::ExternalError;

#[derive(mlua::UserData)]
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

    pub fn encode(&self,context: &mlua::Lua,format: String) -> mlua::Result<mlua::Buffer>{
        if format.to_ascii_uppercase().trim() != "WAV"{
            return Err("encoding non .wav files is unimplemetned".into_lua_err())
        }


        let spec = hound::WavSpec{
            channels : self.channels as u16,
            sample_rate : self.sample_rate,
            bits_per_sample : 32,
            sample_format : hound::SampleFormat::Float,
        };

        let mut cursor = Cursor::new(Vec::new());
        let mut writer = hound::WavWriter::new(&mut cursor,spec).map_err(|err|err.into_lua_err())?;

        for sample_index in 0..self.data[0].len() as usize{
            for channel in &self.data {
                writer.write_sample(channel[sample_index]).map_err(|err|err.into_lua_err())?;
            }
        }

        writer.finalize().map_err(|err|err.into_lua_err())?;

        context.create_buffer(cursor.into_inner())
    }

   #[lua(meta,name = "__tostring")]
    pub fn to_string(&self) -> mlua::Result<String>{
        Ok(format!("audio {}ch {}Hz {} frames",self.channels,self.sample_rate,self.frames))
    }
}