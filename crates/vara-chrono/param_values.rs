use mlua::ExternalError;
// use vara_util::table_builder::TableBuilder;

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
        verify_in_range("Year", self.years, -9999, 9999)?;
        verify_in_range("Month", self.months, 1, 12)?;
        verify_in_range("Day", self.days, 1, 31)?;
        verify_in_range("Hour", self.hours, 0, 24)?;
        verify_in_range("Minute", self.minutes, 0, 59)?;
        verify_in_range("Second", self.seconds, 0, 59)?;
        verify_in_range("Millisecond", self.milliseconds, 0, 999)?;
        Ok(self)
    }
}

fn verify_in_range<T>(name: &str, value: T, min: T, max: T) -> mlua::Result<()>
where
    T: PartialOrd + std::fmt::Display,
{
    assert!(max > min);
    if value <= max && value >= min {
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
            years: value.get("Year")?,
            months: value.get("Month")?,
            days: value.get("Day")?,

            hours: value.get("Hour")?,
            minutes: value.get("Minute")?,
            seconds: value.get("Second")?,
            milliseconds: value.get("Millisecond").unwrap_or(0),
        };

        match parameters.verify() {
            Ok(param) => Ok(param),
            Err(err) => Err(err),
        }
    }
}
