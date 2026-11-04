use std::sync::LazyLock;
use std::sync::Mutex;

use mlua::ExternalError;
use vara_util::table_builder::TableBuilder;
use crate::test::assertion::AssertRecord;

use super::assertion::AssertionEngine;

static ASSERTION_GLOBAL: LazyLock<Mutex<AssertionEngine>> = LazyLock::new(|| Mutex::new(AssertionEngine::new()));

fn assert_eq(_: &mlua::Lua,(expected,actual) : (mlua::Value,mlua::Value)) -> mlua::Result<()>{

    let result = expected.equals(&actual)?;

    if result == false {
        let mut assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

        assert_mutex.record_assert(AssertRecord {
            message: format!("expected {} got {}",expected.to_string()?,actual.to_string()?),
            expected: expected.to_string()?,
            actual: actual.to_string()?,
        });
    }

    Ok(())
}

pub fn get_assertion_engine() ->  mlua::Result<std::sync::MutexGuard<'static,AssertionEngine>> {
    let assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

    Ok(assert_mutex)
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table>{
    TableBuilder::new(context.clone())?
    .add_function("AssertEquals", assert_eq)?
    .build_readonly()
}