pub mod datetime;
mod param_values;
use std::time::{SystemTime, UNIX_EPOCH};

use vara_util::table_builder::TableBuilder;

use datetime::DateTime;
use param_values::DateTimeParams;

fn now(_: &mlua::Lua, _: ()) -> mlua::Result<DateTime> {
    let sys_time: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| mlua::Error::external(e))?
        .as_millis()
        .try_into()
        .map_err(|e| mlua::Error::external(e))?;

    #[cfg(target_family = "windows")]
    let timeoffset = vara_util::win_compat::get_timezone_offset();

    #[cfg(target_family = "unix")]
    let timeoffset = 0;

    Ok(DateTime::new(sys_time, timeoffset))
}

fn from_unix(_: &mlua::Lua, unix_time: i64) -> mlua::Result<DateTime> {
    Ok(DateTime::new(unix_time * 1000, 0))
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_function("now", now)?
        .add_function("fromUnix", from_unix)?
        .add_function("fromUniversal", |_, params: DateTimeParams| {
            Ok(DateTime::from_params(&params, 0))
        })?
        .add_function("fromLocal", |_, params: DateTimeParams| {
            #[cfg(target_family = "windows")]
            let timeoffset = vara_util::win_compat::get_timezone_offset();

            #[cfg(target_family = "unix")]
            let timeoffset = 0;

            Ok(DateTime::from_params(&params, timeoffset))
        })?
        .build_readonly()
}
