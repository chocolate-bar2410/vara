use mlua::ExternalError;
use vara_util::table_builder::TableBuilder;

#[derive(Debug, Clone, Copy)]
pub struct DateTimeParams {
    pub years: i32,
    pub months: u32,
    pub days: u32,

    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}
impl DateTimeParams {
    pub fn verify(self) -> mlua::Result<Self> {
        verify_in_range("years", self.years, 0, 0)?;
        verify_in_range("months", self.months, 0, 0)?;
        verify_in_range("days", self.days, 0, 0)?;
        verify_in_range("hours", self.hours, 0, 0)?;
        verify_in_range("minutes", self.minutes, 0, 0)?;
        verify_in_range("seconds", self.seconds, 0, 0)?;
        verify_in_range("milliseconds", self.milliseconds, 0, 0)?;
        Ok(self)
    }
}

fn verify_in_range<T>(name: &str, value: T, min: T, max: T) -> mlua::Result<()>
where
    T: PartialOrd + std::fmt::Display,
{
    assert!(max > min);
    if (value <= max && value >= min) {
        return Ok(());
    }

    Err(format!("{name} with value {value} did not fall in the range {max} {min}").into_lua_err())
}

impl mlua::FromLua for DateTimeParams {
    fn from_lua(value: mlua::Value, _: &mlua::Lua) -> mlua::Result<Self> {
        if !value.is_table() {
            return Err("value must be a table".into_lua_err());
        }

        let value = value.as_table().unwrap();
        let parameters = Self {
            years: value.get("years")?,
            months: value.get("months")?,
            days: value.get("days")?,

            hours: value.get("hours")?,
            minutes: value.get("minutes")?,
            seconds: value.get("seconds")?,
            milliseconds: value.get("milliseconds").unwrap_or(0),
        };

        match parameters.verify() {
            Ok(param) => Ok(param),
            Err(err) => Err(err),
        }
    }
}
