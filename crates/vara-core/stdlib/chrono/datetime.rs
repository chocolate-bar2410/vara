use mlua::IntoLua;

pub struct DateTime {
    unix_ms: u64,
    timezone_offset: i32,
}

impl DateTime {
    pub fn new(unix_ms: u64, offset: i32) -> Self {
        Self {
            unix_ms: unix_ms,
            timezone_offset: offset,
        }
    }
}

impl mlua::UserData for DateTime {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(
            mlua::MetaMethod::ToString,
            |context: &mlua::Lua, this: &DateTime, _: ()| {
                let total_seconds = (this.unix_ms / 1000) as i64 + this.timezone_offset as i64;
                let seconds_of_day = total_seconds.rem_euclid(86400);

                let hour = seconds_of_day.div_euclid(3600);
                let minute = (seconds_of_day % 3600) / 60;
                let seconds = seconds_of_day % 60;

                let result =
                    format!("{:02}:{:02}:{:02}-00:00", hour, minute, seconds).into_lua(context);

                Ok(result)
            },
        );
    }
}
