use vara_util::table_builder::TableBuilder;

use crate::ffi_library::ForiegnLibrary;

mod ffi_library;

#[cfg(target_os = "windows")]
const SUFFIX: &str = "dll";
#[cfg(target_os = "macos")]
const SUFFIX: &str = "dylib";
#[cfg(target_os = "linux")]
const SUFFIX: &str = "so";

fn dl_open(_: &mlua::Lua,path: String) -> mlua::Result<ffi_library::ForiegnLibrary>{
    ForiegnLibrary::new(path)
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {

    TableBuilder::new(context.clone())?
        .add_value("Suffix", SUFFIX)?
        .add_function("DlOpen", dl_open)?
        .build_readonly()
}