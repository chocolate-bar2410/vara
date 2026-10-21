use std::{ops::Mul};

#[derive(mlua::UserData)]
pub struct Colour4 {
    #[lua(name="Red")]
    red : f32,
    #[lua(name="Green")]
    green : f32,
    #[lua(name="Blue")]
    blue : f32,
    #[lua(name="Alpha")]
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

        let mut red = if self.red <= 0.0031308 {self.red * 12.92} else {self.red.powf(1.0/2.4) * 1.055 - 0.055};
        let mut green = if self.green <= 0.0031308 {self.green * 12.92} else {self.green.powf(1.0/2.4) * 1.055 - 0.055};
        let mut blue = if self.blue <= 0.0031308 {self.blue * 12.92} else {self.blue.powf(1.0/2.4) * 1.055 - 0.055};

        red = red.clamp(0.0,1.0).mul(255.0).round();
        green = green.clamp(0.0,1.0).mul(255.0).round();
        blue = blue.clamp(0.0,1.0).mul(255.0).round();

        Ok(format!("\x1b[38;2;{};{};{}m",red,green,blue).to_string())
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn ansi_conversion(){
        assert_eq!(Colour4::from_srgba(0.5, 0.5, 0.5,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;128;128;128m"));
        assert_eq!(Colour4::from_srgba(1.0, 1.0, 1.0,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;255;255;255m"));
        assert_eq!(Colour4::from_srgba(0.0, 0.0, 0.0,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;0;0;0m"));

        assert_eq!(Colour4::from_srgba(0.001, 0.002, 0.003,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;0;1;1m"));
        assert_eq!(Colour4::from_srgba(0.999, 0.999, 0.999,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;255;255;255m"));

        assert_eq!(Colour4::from_srgba(0.04045, 0.04045, 0.04045,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;10;10;10m"),"below threshold");
        assert_eq!(Colour4::from_srgba(0.04046, 0.04046, 0.04046,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;10;10;10m"),"above threshold");

        assert_eq!(Colour4::from_srgba(-0.1, -0.1, -0.1,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;0;0;0m"),"below 0 colour");
        assert_eq!(Colour4::from_srgba(2.0, 2.0, 2.0,1.0).to_ansi().unwrap(),String::from("\x1b[38;2;255;255;255m"),"hdr colour");
    }
}

