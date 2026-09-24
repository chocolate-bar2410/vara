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
}