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
            message: format!("{} is not equal to {}",expected.to_string()?,actual.to_string()?),
        });
    }

    Ok(())
}

fn assert_neq(_: &mlua::Lua,(expected,actual) : (mlua::Value,mlua::Value)) -> mlua::Result<()>{

    let result = !expected.equals(&actual)?;

    if result == false {
        let mut assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

        assert_mutex.record_assert(AssertRecord {
            message: format!("{} is equal to {}",expected.to_string()?,actual.to_string()?),
        });
    }

    Ok(())
}

fn assert_true(_: &mlua::Lua,condition : bool) -> mlua::Result<()>{

    if !condition {
        let mut assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

        assert_mutex.record_assert(AssertRecord {
            message: format!("{} is not true",condition),
        });
    }

    Ok(())
}

fn assert_not_nil(_: &mlua::Lua,value : mlua::Value) -> mlua::Result<()>{

    if value == mlua::Value::Nil {
        let mut assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

        assert_mutex.record_assert(AssertRecord {
            message: format!("{} is not nil",value.to_string()?),
        });
    }

    Ok(())
}

fn assert_throw(_: &mlua::Lua,(callback_fn,args) : (mlua::Function,mlua::MultiValue)) -> mlua::Result<()>{

    let result = callback_fn.call::<()>(args);

    match result {
        Err(_) => {},
        Ok(_) => {
            let mut assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

            assert_mutex.record_assert(AssertRecord {
                message: format!("{} did not throw an error",mlua::Value::Function(callback_fn).to_string()?),
            });
        }
    }

    Ok(())
}

fn assert_not_throw(_: &mlua::Lua,(callback_fn,args) : (mlua::Function,mlua::MultiValue)) -> mlua::Result<()>{

    let result = callback_fn.call::<()>(args);

    match result {
        Ok(_) => {},
        Err(_) => {
            let mut assert_mutex = ASSERTION_GLOBAL.lock().map_err(|_| "poisioned assertion engine mutex".into_lua_err())?;

            assert_mutex.record_assert(AssertRecord {
                message: format!("{} threw an error",mlua::Value::Function(callback_fn).to_string()?),
            });
        }
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
    .add_function("AssertNotEquals", assert_neq)?
    .add_function("AssertTrue", assert_true)?
    .add_function("AssertNotNil", assert_not_nil)?
    .add_function("AssertThrows", assert_throw)?
    .add_function("AssertDoesNotThrow", assert_not_throw)?
    .build_readonly()
}