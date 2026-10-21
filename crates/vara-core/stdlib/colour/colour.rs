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

        let (mut red,mut green,mut blue,_) = self.to_srgba()?;
        
        red = red.clamp(0.0,1.0).mul(255.0).round();
        green = green.clamp(0.0,1.0).mul(255.0).round();
        blue = blue.clamp(0.0,1.0).mul(255.0).round();

        Ok(format!("\x1b[38;2;{};{};{}m",red,green,blue).to_string())
    }

    #[lua(name="ToSRGBA")]
    pub fn to_srgba(&self) -> mlua::Result<(f32,f32,f32,f32)>{
        let red = if self.red <= 0.0031308 {self.red * 12.92} else {self.red.powf(1.0/2.4) * 1.055 - 0.055};
        let green = if self.green <= 0.0031308 {self.green * 12.92} else {self.green.powf(1.0/2.4) * 1.055 - 0.055};
        let blue = if self.blue <= 0.0031308 {self.blue * 12.92} else {self.blue.powf(1.0/2.4) * 1.055 - 0.055};

        Ok((red,green,blue,self.alpha))
    }

    #[lua(name="ToHex")]
    pub fn to_hex(&self) -> mlua::Result<String>{
        let (mut red,mut green,mut blue,mut alpha) = self.to_srgba()?;

        red = red.clamp(0.0,1.0).mul(255.0).round();
        green = green.clamp(0.0,1.0).mul(255.0).round();
        blue = blue.clamp(0.0,1.0).mul(255.0).round();
        alpha = alpha.clamp(0.0,1.0).mul(255.0).round();

        let hex_string = [red,green,blue,alpha].map(|num| format!("{:02X}",num as u8)).concat();

        Ok(hex_string)
    }

    #[lua(name="ToHSVA")]
    pub fn to_hsva(&self) -> mlua::Result<(f32,f32,f32,f32)>{
        let (mut red,mut green,mut blue,_) = self.to_srgba()?;

        red = red.max(0.0);
        green = green.max(0.0);
        blue = blue.max(0.0);

        let epsilon = 1.0e-20f32;

        let c_min = red.min(green).min(blue);
        let c_max = red.max(green).max(blue);

        let chroma = c_max - c_min;
        let inv_chroma = 1.0 / (6.0 * chroma + epsilon);

        // sector masks, to reduce branching

        let is_r = ((c_max == red) as u32) as f32;
        let is_g = ((c_max == green && c_max != red) as u32) as f32;
        let is_b = ((c_max == blue && c_max != green && c_max != red) as u32) as f32;
        
        let hr = (green - blue) * inv_chroma;
        let hg = (blue - red) * inv_chroma + 1.0 / 3.0;
        let hb = (red - green) * inv_chroma + 2.0 / 3.0;

        let mut hue = (hr * is_r + hg * is_g + hb * is_b).rem_euclid(1.0);

        let saturation = if c_max > 0.0 {
            chroma / c_max
        } else {
            0.0
        };

        hue = if chroma == 0.0 {0.0} else {hue};

        Ok((hue * 360.0,saturation,c_max,self.alpha))
    }

}

#[cfg(test)]
mod tests{
    use super::*;

    fn compare_srgba_approx_eq(result: (f32,f32,f32,f32),expected: (f32,f32,f32,f32),messsage: &str){

        let tolerance = 1e-4;

        for(result,expected) in [(result.0,expected.0),(result.1,expected.1),(result.2,expected.2),(result.3,expected.3)]{
            assert!((result - expected).abs() < tolerance,"{messsage}\nexpected {expected} got {result}")
        }
    }

    #[test]
    fn srgb_conversion() -> Result<(),Box<dyn std::error::Error>>{
        
        compare_srgba_approx_eq(Colour4::from_srgba(0.5, 0.5, 0.5,1.0).to_srgba()?,(0.5, 0.5, 0.5,1.0),"gray srgba");
        compare_srgba_approx_eq(Colour4::from_srgba(1.0, 1.0, 1.0,1.0).to_srgba()?,(1.0, 1.0, 1.0,1.0),"white srgba");
        compare_srgba_approx_eq(Colour4::from_srgba(0.0, 0.0, 0.0,1.0).to_srgba()?,(0.0, 0.0, 0.0,1.0),"black srgba");

        compare_srgba_approx_eq(Colour4::from_srgba(0.001, 0.002, 0.003,1.0).to_srgba()?,(0.001, 0.002, 0.003,1.0),"near black srgba");
        compare_srgba_approx_eq(Colour4::from_srgba(0.999, 0.999, 0.999,1.0).to_srgba()?,(0.999, 0.999, 0.999,1.0),"near white srgba");

        compare_srgba_approx_eq(Colour4::from_srgba(0.04045, 0.04045, 0.04045,1.0).to_srgba()?,(0.04045, 0.04045, 0.04045,1.0),"below threshold");
        compare_srgba_approx_eq(Colour4::from_srgba(0.04046, 0.04046, 0.04046,1.0).to_srgba()?,(0.04046, 0.04046, 0.04046,1.0),"above threshold");

        compare_srgba_approx_eq(Colour4::from_srgba(-0.1, -0.1, -0.1,1.0).to_srgba()?,(-0.1, -0.1, -0.1,1.0),"below 0 colour");
        compare_srgba_approx_eq(Colour4::from_srgba(2.0, 2.0, 2.0,1.0).to_srgba()?,(2.0, 2.0, 2.0,1.0),"hdr colour");

        Ok(())
    }

    #[test]
    fn ansi_conversion() -> Result<(),Box<dyn std::error::Error>>{
        assert_eq!(Colour4::from_srgba(0.5, 0.5, 0.5,1.0).to_ansi()?,String::from("\x1b[38;2;128;128;128m"));
        assert_eq!(Colour4::from_srgba(1.0, 1.0, 1.0,1.0).to_ansi()?,String::from("\x1b[38;2;255;255;255m"));
        assert_eq!(Colour4::from_srgba(0.0, 0.0, 0.0,1.0).to_ansi()?,String::from("\x1b[38;2;0;0;0m"));

        assert_eq!(Colour4::from_srgba(0.001, 0.002, 0.003,1.0).to_ansi()?,String::from("\x1b[38;2;0;1;1m"));
        assert_eq!(Colour4::from_srgba(0.999, 0.999, 0.999,1.0).to_ansi()?,String::from("\x1b[38;2;255;255;255m"));

        assert_eq!(Colour4::from_srgba(0.04045, 0.04045, 0.04045,1.0).to_ansi()?,String::from("\x1b[38;2;10;10;10m"),"below threshold");
        assert_eq!(Colour4::from_srgba(0.04046, 0.04046, 0.04046,1.0).to_ansi()?,String::from("\x1b[38;2;10;10;10m"),"above threshold");

        assert_eq!(Colour4::from_srgba(-0.1, -0.1, -0.1,1.0).to_ansi()?,String::from("\x1b[38;2;0;0;0m"),"below 0 colour");
        assert_eq!(Colour4::from_srgba(2.0, 2.0, 2.0,1.0).to_ansi()?,String::from("\x1b[38;2;255;255;255m"),"hdr colour");

        Ok(())
    }

    #[test]
    fn hex_conversion() -> Result<(),Box<dyn std::error::Error>> {
        assert_eq!(Colour4::from_srgba(0.5, 0.5, 0.5,1.0).to_hex()?,String::from("808080FF"));
        assert_eq!(Colour4::from_srgba(1.0, 1.0, 1.0,1.0).to_hex()?,String::from("FFFFFFFF"));
        assert_eq!(Colour4::from_srgba(0.0, 0.0, 0.0,1.0).to_hex()?,String::from("000000FF"));

        assert_eq!(Colour4::from_srgba(0.001, 0.002, 0.003,1.0).to_hex()?,String::from("000101FF"));
        assert_eq!(Colour4::from_srgba(0.999, 0.999, 0.999,1.0).to_hex()?,String::from("FFFFFFFF"));

        assert_eq!(Colour4::from_srgba(0.04045, 0.04045, 0.04045,1.0).to_hex()?,String::from("0A0A0AFF"),"below threshold");
        assert_eq!(Colour4::from_srgba(0.04046, 0.04046, 0.04046,1.0).to_hex()?,String::from("0A0A0AFF"),"above threshold");

        assert_eq!(Colour4::from_srgba(-0.1, -0.1, -0.1,1.0).to_hex()?,String::from("000000FF"),"below 0 colour");
        assert_eq!(Colour4::from_srgba(2.0, 2.0, 2.0,1.0).to_hex()?,String::from("FFFFFFFF"),"hdr colour");
    
        Ok(())
    }
    #[test]
    fn hsv_conversion() -> Result<(),Box<dyn std::error::Error>> {
        compare_srgba_approx_eq(Colour4::from_srgba(0.5, 0.5, 0.5,1.0).to_hsva()?,(0.0,0.0,0.5,1.0),"gray hsva");
        compare_srgba_approx_eq(Colour4::from_srgba(1.0, 1.0, 1.0,1.0).to_hsva()?,(0.0, 0.0, 1.0,1.0),"white hsva");
        compare_srgba_approx_eq(Colour4::from_srgba(0.0, 0.0, 0.0,1.0).to_hsva()?,(0.0, 0.0, 0.0,1.0),"black hsva");

        compare_srgba_approx_eq(Colour4::from_srgba(0.001, 0.002, 0.003,1.0).to_hsva()?,(210.0,2.0/3.0,0.003,1.0),"near black hsva");
        compare_srgba_approx_eq(Colour4::from_srgba(0.999, 0.999, 0.999,1.0).to_hsva()?,(0.0,0.0,0.999,1.0),"near white hsva");

        compare_srgba_approx_eq(Colour4::from_srgba(0.04045, 0.04045, 0.04045,1.0).to_hsva()?,(0.0,0.0,0.04045,1.0),"below threshold");
        compare_srgba_approx_eq(Colour4::from_srgba(0.04046, 0.04046, 0.04046,1.0).to_hsva()?,(0.0,0.0,0.04046,1.0),"above threshold");

        compare_srgba_approx_eq(Colour4::from_srgba(-0.1, -0.1, -0.1,1.0).to_hsva()?,(0.0, 0.0, 0.0,1.0),"below 0 colour");
        compare_srgba_approx_eq(Colour4::from_srgba(2.0, 2.0, 2.0,1.0).to_hsva()?,(0.0, 0.0, 2.0,1.0),"hdr colour");

        Ok(())
    }
}

