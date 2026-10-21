use std::{ops::Mul};

#[derive(mlua::UserData)]
pub struct Colour4 {
    red : f32,
    green : f32,
    blue : f32,
    alpha : f32,
}

#[mlua::userdata_impl]
impl Colour4 {
    #[lua(skip)]
    pub fn new(red : f32,green : f32,blue : f32,alpha : f32) -> Colour4{
        Self {
            red,
            green,
            blue,
            alpha,
        }
    } 

    #[lua(skip)]
    pub fn from_srgba(red : f32,green : f32,blue : f32,alpha : f32) -> Colour4{

        let red = if red <= 0.04045 {red / 12.92} else {((red + 0.055)/1.055).powf(2.4)};
        let green = if green <= 0.04045 {green / 12.92} else {((green + 0.055)/1.055).powf(2.4)};
        let blue = if blue <= 0.04045 {blue / 12.92} else {((blue + 0.055)/1.055).powf(2.4)};

        Self {
            red,
            green,
            blue,
            alpha,
        }
    } 

    #[lua(name="ToAnsi")]
    pub fn to_ansi(&self) -> mlua::Result<String>{
        let red  = self.red.clamp(0.0,1.0).mul(255.0).floor();
        let green = self.green.clamp(0.0,1.0).mul(255.0).floor();
        let blue = self.blue.clamp(0.0,1.0).mul(255.0).floor();

        Ok(format!("\x1b[38;2;{};{};{}m",red,green,blue).to_string())
    }

}


