pub fn image_format_fromstring(value: &String) -> mlua::Result<image::ImageFormat> {
    match value.to_ascii_uppercase().trim() {
        "PNG" => Ok(image::ImageFormat::Png),
        "JPEG" => Ok(image::ImageFormat::Jpeg),
        "WEBP" => Ok(image::ImageFormat::WebP),
        "PNM" => Ok(image::ImageFormat::Pnm),
        "BMP" => Ok(image::ImageFormat::Bmp),
        "QOI" => Ok(image::ImageFormat::Qoi),
        _ => Err(mlua::Error::FromLuaConversionError {
            from: "String",
            to: "ImageFormat".to_string(),
            message: Some("Invalid format".to_string()),
        }),
    }
}