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
    unix_ms: i64,

    // in seconds
    timezone_offset: i32,

    // precomputed values so i dont have to recalculate everything
    years: i64,
    months: i64,
    days: i64,

    hours: i64,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
}

impl DateTime {
    pub fn new(unix_ms: i64, offset: i32) -> Self {
        let mut new_date_time = DateTime {
            unix_ms: unix_ms,
            timezone_offset: offset,
            years: 0,
            months: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        };

        // precompute date and time for further use
        new_date_time.compute_date();
        new_date_time.compute_time();

        new_date_time
    }

    fn compute_date(&mut self) {
        let total_seconds = (self.unix_ms / 1000) + self.timezone_offset as i64;
        let days_since_unix = total_seconds.div_euclid(SECONDS_PER_DAY);

        let days_since_march_epoch = days_since_unix + UNIX_EPOCH_TO_MARCH_EPOCH;

        // see https://howardhinnant.github.io/date_algorithms.html#days_from_civil
        // for information on date calculations

        let era = days_since_march_epoch.div_euclid(DAYS_PER_ERA);

        let day_of_era = days_since_march_epoch - era * DAYS_PER_ERA;
        let year_of_era = (day_of_era - day_of_era / (DAYS_PER_QUADRENNIA - 1)
            + day_of_era / DAYS_PER_CENTURY
            - day_of_era / (DAYS_PER_ERA - 1))
            .div_euclid(DAYS_PER_YEAR);

        let day_of_year =
            day_of_era - (DAYS_PER_YEAR * year_of_era + year_of_era / 4 - year_of_era / 100);

        let march_based_month = (5 * day_of_year + 2) / 153;

        self.years = year_of_era + era * YEARS_PER_ERA;
        self.months = march_based_month + if march_based_month < 10 { 3 } else { -9 };
        self.days = day_of_year - (153 * march_based_month + 2) / 5 + 1;

        if self.months <= 2 {
            self.years += 1;
        }
    }
    fn compute_time(&mut self) {
        let total_seconds = (self.unix_ms / 1000) + self.timezone_offset as i64;
        let seconds_of_day = total_seconds.rem_euclid(SECONDS_PER_DAY);

        self.hours = seconds_of_day.div_euclid(SECONDS_PER_HOUR);
        self.minutes = (seconds_of_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
        self.seconds = seconds_of_day % SECONDS_PER_MINUTE;
        self.milliseconds = self.unix_ms.rem_euclid(1000);
    }

    pub fn to_iso8601(&self) -> String {
        let offset_hours = self.timezone_offset.div_euclid(SECONDS_PER_HOUR as i32);
        let offset_minutes = self
            .timezone_offset
            .rem_euclid(SECONDS_PER_HOUR as i32)
            .div_euclid(SECONDS_PER_MINUTE as i32);

        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{:+03}:{:02}",
            self.years,
            self.months,
            self.days,
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds,
            offset_hours,
            offset_minutes
        )
    }
}

impl mlua::UserData for DateTime {
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "toISO8601",
            |context: &mlua::Lua, this: &DateTime, _: ()| Ok(this.to_iso8601().into_lua(context)),
        );

        methods.add_meta_method(
            mlua::MetaMethod::ToString,
            |context: &mlua::Lua, this: &DateTime, _: ()| Ok(this.to_iso8601().into_lua(context)),
        );
    }
}
