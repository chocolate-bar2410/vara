use vara_util::table_builder::TableBuilder;

#[cfg(target_os = "windows")]
const SUFFIX: &str = "dll";
#[cfg(target_os = "macos")]
const SUFFIX: &str = "dylib";
#[cfg(target_os = "linux")]
const SUFFIX: &str = "so";

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {

    TableBuilder::new(context.clone())?
        .add_value("Suffix", SUFFIX)?
        .build_readonly()
}