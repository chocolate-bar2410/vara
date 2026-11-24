use mlua::ExternalResult;
use vara_media::Image;

#[derive(mlua::UserData)]
pub struct Clipboard{
    #[lua(skip)]
    handle: arboard::Clipboard
}

#[mlua::userdata_impl]
impl Clipboard{

    #[lua(skip)]
    pub fn new() -> mlua::Result<Self>{
        Ok(Self { handle:  arboard::Clipboard::new().into_lua_err()?})
    }

    #[lua(name="GetText")]
    pub fn get_text(&mut self) -> mlua::Result<Option<String>>{
        match self.handle.get_text(){
            Ok(text) => Ok(Some(text)),
            Err(_) => Ok(None)
        }
    }

    #[lua(name="SetText")]
    pub fn set_text(&mut self,text: String) -> mlua::Result<()>{
        self.handle.set_text(text).into_lua_err()
    }

    #[lua(name="GetImage")]
    pub fn get_image(&mut self) -> mlua::Result<Image>{
        let image_data = self.handle.get_image().into_lua_err()?;

        let image_bytes = image_data.bytes
        .chunks_exact(4)
        .flat_map(|pixel| {
            [
                srgb_to_linear(pixel[0] as f32 / 255.0),
                srgb_to_linear(pixel[1] as f32 / 255.0),
                srgb_to_linear(pixel[2] as f32 / 255.0),
                pixel[3] as f32 / 255.0
            ]
        })
        .collect();

        let image = Image::new(image_bytes,image_data.width as u32,image_data.height as u32,4);

        Ok(image)
    }

    #[lua(name="SetImage")]
    pub fn set_image(&mut self,image_userdata: mlua::Value) -> mlua::Result<()>{
        
        match image_userdata{
            mlua::Value::UserData(image_userdata) => {
                let image = image_userdata.borrow::<Image>()?;

                let image_bytes = image.data
                .chunks_exact(4)
                .flat_map(|pixel| {
                    [
                        (linear_to_srgb(pixel[0]) * 255.0).clamp(0.0,255.0).round() as u8,
                        (linear_to_srgb(pixel[1]) * 255.0).clamp(0.0,255.0).round() as u8,
                        (linear_to_srgb(pixel[2]) * 255.0).clamp(0.0,255.0).round() as u8,
                        (pixel[3] * 255.0) as u8
                    ]
                })
                .collect();

                self.handle.set_image(arboard::ImageData { width: image.width as usize, height: image.height as usize, bytes: image_bytes }).into_lua_err()?;

                Ok(())
            },
            _ => return Err(mlua::Error::FromLuaConversionError {
                from: image_userdata.type_name(),
                to: "Image".to_string(),
                message: Some("expected a Image".to_string()),
            })
        }
    }
}

pub fn srgb_to_linear(value: f32) -> f32{
    if value <= 0.04045f32 {value / 12.92f32} else {((value + 0.055f32)/1.055).powf(2.4)}
}

pub fn linear_to_srgb(value: f32) -> f32 {
    if value <= 0.0031308 {value * 12.92} else {value.powf(1.0/2.4) * 1.055 - 0.055}
}