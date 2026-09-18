use mlua::IntoLua;

const SECONDS_PER_MINUTE: i64 = 60;
const MINUTES_PER_HOUR: i64 = 60;

const SECONDS_PER_HOUR: i64 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

const SECONDS_PER_DAY: i64 = 86400;
const DAYS_PER_YEAR: i64 = 365;
const YEARS_PER_ERA: i64 = 400;

const DAYS_PER_QUADRENNIA: i64 = 1461;
const DAYS_PER_CENTURY: i64 = 36524;
const DAYS_PER_ERA: i64 = 146097;

const UNIX_EPOCH_TO_MARCH_EPOCH: i64 = 719468;

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
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {}

    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(
            mlua::MetaMethod::ToString,
            |context: &mlua::Lua, this: &DateTime, _: ()| {
                let total_seconds = (this.unix_ms / 1000) as i64 + this.timezone_offset as i64;
                let seconds_of_day = total_seconds.rem_euclid(SECONDS_PER_DAY);
                let days_since_unix = total_seconds.div_euclid(SECONDS_PER_DAY);

                let days_since_march_epoch = days_since_unix + UNIX_EPOCH_TO_MARCH_EPOCH;

                let hour = seconds_of_day.div_euclid(SECONDS_PER_HOUR);
                let minute = (seconds_of_day % SECONDS_PER_HOUR) / MINUTES_PER_HOUR;
                let seconds = seconds_of_day % SECONDS_PER_MINUTE;

                // see https://howardhinnant.github.io/date_algorithms.html#days_from_civil
                // for information on date calculations

                let era = days_since_march_epoch.div_euclid(DAYS_PER_ERA);

                let day_of_era = days_since_march_epoch - era * DAYS_PER_ERA;
                let year_of_era = (day_of_era - day_of_era / (DAYS_PER_QUADRENNIA - 1)
                    + day_of_era / DAYS_PER_CENTURY
                    - day_of_era / (DAYS_PER_ERA - 1))
                    .div_euclid(DAYS_PER_YEAR);

                let mut years = year_of_era + era * YEARS_PER_ERA;

                let day_of_year = day_of_era
                    - (DAYS_PER_YEAR * year_of_era + year_of_era / 4 - year_of_era / 100);

                let march_based_month = (5 * day_of_year + 2) / 153;

                let day = day_of_year - (153 * march_based_month + 2) / 5 + 1;

                let month = march_based_month + if march_based_month < 10 { 3 } else { -9 };

                if month <= 2 {
                    years += 1;
                }

                let result = format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}+00:00",
                    years, month, day, hour, minute, seconds
                )
                .into_lua(context);

                Ok(result)
            },
        );
    }
}
