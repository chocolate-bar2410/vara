use std::sync::LazyLock;
use std::sync::Mutex;

use vara_util::table_builder::TableBuilder;
use super::assertion::AssertionEngine;

static assertion: LazyLock<Mutex<AssertionEngine>> = LazyLock::new(|| Mutex::new(AssertionEngine::new()));

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table>{
    TableBuilder::new(context.clone())?.build_readonly()
}