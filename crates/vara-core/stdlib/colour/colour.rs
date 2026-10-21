use std::{fmt::format, ops::Mul};

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

    #[lua(name="ToAnsi")]
    pub fn to_ansi(&self) -> mlua::Result<String>{
        let red  = self.red.clamp(0.0,1.0).mul(255.0).floor();
        let green = self.green.clamp(0.0,1.0).mul(255.0).floor();
        let blue = self.blue.clamp(0.0,1.0).mul(255.0).floor();

        Ok(format!("\x1b[38;2;{};{};{}m",red,green,blue).to_string())
    }

}


