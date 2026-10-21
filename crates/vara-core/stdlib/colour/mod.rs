use vara_util::table_builder::TableBuilder;

pub mod colour;

pub fn new(_: &mlua::Lua,(red,green,blue,alpha):(f32,f32,f32,Option<f32>)) -> mlua::Result<colour::Colour4>{

    let alpha = alpha.unwrap_or(1.0);

    Ok(colour::Colour4::new(red, green, blue, alpha))
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
    .add_function("New", new)?
    .build_readonly()
}
