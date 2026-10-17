
use std::path::{Path, PathBuf};

use mlua::{ExternalResult, LuaSerdeExt};
use vara_util::table_builder::TableBuilder;

pub fn join(context: &mlua::Lua,paths: mlua::MultiValue) -> mlua::Result<String>{

    let path_items: mlua::Result<Vec<String>> = paths.into_vec()
        .into_iter()
        .map(|value| 
            context.from_value::<String>(value)
        )
        .collect();

    #[cfg(target_family = "unix")]
    return Ok(path_items?.join("/"));

    #[cfg(target_family = "windows")]
    Ok(path_items?.join("\\"))
}

pub fn with_ext(_: &mlua::Lua,(path,ext): (String,String)) -> mlua::Result<String>{
    Ok(format!("{}.{}",path,ext))
}

pub fn base_name(_: &mlua::Lua,path: String) -> mlua::Result<String>{
    let path_obj = Path::new(&path);

    Ok(path_obj
        .file_name()
        .and_then(|os_str| Some(os_str.to_string_lossy().into_owned()))
        .unwrap_or(String::from(""))
    )
}

pub fn absolute(_: &mlua::Lua,path: String) -> mlua::Result<PathBuf>{
    std::path::absolute(&path).into_lua_err()
}

pub fn glob(_: &mlua::Lua,(path,pattern): (String,String)) -> mlua::Result<bool>{
    Ok(fast_glob::glob_match(path, pattern))
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("Join", join)?
        .add_function("WithExt", with_ext)?
        .add_function("Basename", base_name)?
        .add_function("Absolute", absolute)?
        .add_function("Glob", glob)?
        .build_readonly()
}
