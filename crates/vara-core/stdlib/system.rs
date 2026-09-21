use std::env::consts::{ARCH, FAMILY, OS};
use vara_util::table_builder::TableBuilder;

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_value("os", OS)?
        .add_value("arch", ARCH)?
        .add_value("family", FAMILY)?
        .add_value("cwd", std::env::current_dir()?)?
        .add_value("temp", std::env::temp_dir())?
        .add_value(
            "endian",
            if cfg!(target_endian = "big") {
                "big"
            } else {
                "little"
            },
        )?
        .build_readonly()
}
