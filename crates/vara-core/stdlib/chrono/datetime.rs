pub struct DateTime {
    unix_ms: i64,
    timezone_offset: i32,
}

impl DateTime {
    pub fn new(unix_ms: i64, offset: i32) -> Self {
        Self {
            unix_ms: unix_ms,
            timezone_offset: offset,
        }
    }
}

impl mlua::UserData for DateTime {}
