use mlua::IntoLua;
use std::cmp::Ordering;

use super::param_values::DateTimeParams;

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

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    unix_ms: i64,

    // in seconds
    time_offset: i32,

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
            time_offset: offset,
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

    pub fn from_params(params: &DateTimeParams, time_offset: i32) -> Self {
        let mut new_date_time = DateTime {
            unix_ms: 0,
            time_offset: time_offset,
            years: params.years as i64,
            months: params.months as i64,
            days: params.days as i64,
            hours: params.hours as i64,
            minutes: params.minutes as i64,
            seconds: params.seconds as i64,
            milliseconds: params.milliseconds as i64,
        };

        // precompute unix for further use
        new_date_time.compute_unix();

        new_date_time
    }

    fn compute_unix(&mut self) {
        let year_corrected = (self.years - if self.months <= 2 { 1 } else { 0 }) as i64;

        let era = (if year_corrected >= 0 {
            year_corrected
        } else {
            year_corrected - 399
        } as i64)
            .div_euclid(YEARS_PER_ERA);

        let year_of_era = (year_corrected - era * YEARS_PER_ERA) as u64;
        let march_based_month = if self.months > 2 {
            self.months - 3
        } else {
            self.months + 9
        };

        let day_of_year = ((153 * march_based_month + 2) / 5 + self.days - 1) as u64;

        let day_of_era = year_of_era * DAYS_PER_YEAR as u64 + year_of_era / 4 as u64
            - year_of_era / 100 as u64
            + day_of_year;

        let days_since_unix: i64 =
            era as i64 * DAYS_PER_ERA + day_of_era as i64 - UNIX_EPOCH_TO_MARCH_EPOCH;

        let seconds_since_unix = days_since_unix * SECONDS_PER_DAY
            + self.hours as i64 * SECONDS_PER_HOUR
            + self.minutes as i64 * SECONDS_PER_MINUTE
            + self.seconds as i64;

        self.unix_ms = seconds_since_unix * 1000 + self.milliseconds as i64;
    }

    fn compute_date(&mut self) {
        let total_seconds = (self.unix_ms.div_euclid(1000)) + self.time_offset as i64;
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
        let total_seconds = (self.unix_ms.div_euclid(1000)) + self.time_offset as i64;
        let seconds_of_day = total_seconds.rem_euclid(SECONDS_PER_DAY);

        self.hours = seconds_of_day.div_euclid(SECONDS_PER_HOUR);
        self.minutes = (seconds_of_day % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
        self.seconds = seconds_of_day % SECONDS_PER_MINUTE;
        self.milliseconds = self.unix_ms.rem_euclid(1000);
    }

    fn get_offset_formatted(&self) -> (char, u32, u32) {
        let sign = if self.time_offset < 0 { '-' } else { '+' };

        let offset_hours = self
            .time_offset
            .unsigned_abs()
            .div_euclid(SECONDS_PER_HOUR as u32);

        let offset_minutes = self
            .time_offset
            .unsigned_abs()
            .rem_euclid(SECONDS_PER_HOUR as u32)
            .div_euclid(SECONDS_PER_MINUTE as u32);

        (sign, offset_hours, offset_minutes)
    }

    pub fn to_iso8601(&self) -> String {
        let (sign, offset_hours, offset_minutes) = self.get_offset_formatted();

        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{sign}{:02}:{:02}",
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

    pub fn to_rfc2822(&self) -> String {
        let (sign, offset_hours, offset_minutes) = self.get_offset_formatted();

        let total_seconds = (self.unix_ms.div_euclid(1000)) + self.time_offset as i64;
        let days_since_unix = total_seconds.div_euclid(SECONDS_PER_DAY);
        let day_of_week = match (days_since_unix + 4).rem_euclid(7) {
            0 => "Sun",
            1 => "Mon",
            2 => "Tue",
            3 => "Wed",
            4 => "Thu",
            5 => "Fri",
            6 => "Sat",
            _ => "???",
        };

        let month_name = match self.months {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Unknown",
        };

        format!(
            "{}, {:02} {} {:04} {:02}:{:02}:{:02} {sign}{:02}{:02}",
            day_of_week,
            self.days,
            month_name,
            self.years,
            self.hours,
            self.minutes,
            self.seconds,
            offset_hours,
            offset_minutes
        )
    }

    pub fn humanise(&self) -> String {
        let (sign, offset_hours, offset_minutes) = self.get_offset_formatted();

        let total_seconds = (self.unix_ms.div_euclid(1000)) + self.time_offset as i64;
        let days_since_unix = total_seconds.div_euclid(SECONDS_PER_DAY);

        let day_of_week = match (days_since_unix + 4).rem_euclid(7) {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => "???",
        };

        let month_name = match self.months {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        };

        format!(
            "{}, {} {} {} {:02}:{:02}:{:02} (UTC{sign}{:1}:{:02})",
            day_of_week,
            self.days,
            month_name,
            self.years,
            self.hours,
            self.minutes,
            self.seconds,
            offset_hours,
            offset_minutes
        )
    }

    pub fn to_zh(&self) -> String {
        let (sign, offset_hours, offset_minutes) = self.get_offset_formatted();

        let total_seconds = (self.unix_ms.div_euclid(1000)) + self.time_offset as i64;
        let days_since_unix = total_seconds.div_euclid(SECONDS_PER_DAY);

        let day_of_week = match (days_since_unix + 4).rem_euclid(7) {
            0 => "星期日",
            1 => "星期一",
            2 => "星期二",
            3 => "星期三",
            4 => "星期四",
            5 => "星期五",
            6 => "星期六",
            _ => "???",
        };

        format!(
            "{}年{}月{}日 {} {:02}时{:02}分{:02}秒 (UTC{sign}{:1}:{:02})",
            self.years,
            self.months,
            self.days,
            day_of_week,
            self.hours,
            self.minutes,
            self.seconds,
            offset_hours,
            offset_minutes
        )
    }

    pub fn to_apache_log(&self) -> String {
        let (sign, offset_hours, offset_minutes) = self.get_offset_formatted();

        let month_name = match self.months {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Unknown",
        };

        format!(
            "[{:02}/{}/{}:{:02}:{:02}:{:02}.{:03} {sign}{:02}{:02}]",
            self.days,
            month_name,
            self.years,
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds,
            offset_hours,
            offset_minutes
        )
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.unix_ms == other.unix_ms
    }
}

impl Eq for DateTime {}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.unix_ms.cmp(&other.unix_ms)
    }
}

impl mlua::UserData for DateTime {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field("TimeOffset", 0);

        fields.add_field_method_set("TimeOffset", |_, this, value: f64| {
            this.time_offset = (SECONDS_PER_HOUR as f64 * value) as i32;

            this.compute_date();
            this.compute_time();

            Ok(())
        });

        fields.add_field_method_get("TimeOffset", |context: &mlua::Lua, this| {
            let result = this.time_offset as f64 / SECONDS_PER_HOUR as f64;

            result.into_lua(context)
        });

        fields.add_field("Year", 0);
        fields.add_field("Month", 0);
        fields.add_field("Day", 0);

        fields.add_field("Hour", 0);
        fields.add_field("Minute", 0);
        fields.add_field("Second", 0);
        fields.add_field("Millisecond", 0);

        fields.add_field_method_get("Year", |_, this| Ok(this.years));
        fields.add_field_method_get("Month", |_, this| Ok(this.months));
        fields.add_field_method_get("Day", |_, this| Ok(this.days));

        fields.add_field_method_get("Hour", |_, this| Ok(this.hours));
        fields.add_field_method_get("minute", |_, this| Ok(this.minutes));
        fields.add_field_method_get("Second", |_, this| Ok(this.seconds));
        fields.add_field_method_get("Millisecond", |_, this| Ok(this.milliseconds));

        fields.add_field_method_set("Year", |_, this, value: i64| {
            this.years = value;
            Ok(this.compute_unix())
        });

        fields.add_field_method_set("Month", |_, this, value: i64| {
            this.months = value;
            Ok(this.compute_unix())
        });

        fields.add_field_method_set("Day", |_, this, value: i64| {
            this.days = value;
            Ok(this.compute_unix())
        });

        fields.add_field_method_set("Hour", |_, this, value: i64| {
            this.hours = value;
            Ok(this.compute_unix())
        });

        fields.add_field_method_set("minute", |_, this, value: i64| {
            this.minutes = value;
            Ok(this.compute_unix())
        });

        fields.add_field_method_set("Second", |_, this, value: i64| {
            this.seconds = value;
            Ok(this.compute_unix())
        });

        fields.add_field_method_set("Millisecond", |_, this, value: i64| {
            this.milliseconds = value;
            Ok(this.compute_unix())
        });
    }
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("ToISO8601", |_, this, ()| Ok(this.to_iso8601()));
        methods.add_method("ToRFC2822", |_, this, ()| Ok(this.to_rfc2822()));

        methods.add_method("Humanise", |_, this, ()| Ok(this.humanise()));
        methods.add_method("ToZH", |_, this, ()| Ok(this.to_zh()));
        methods.add_method("ToApacheLog", |_, this, ()| Ok(this.to_apache_log()));

        methods.add_meta_method(mlua::MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_iso8601())
        });

        methods.add_meta_method(
            mlua::MetaMethod::Eq,
            |_, this, other: mlua::UserDataRef<Self>| Ok(this.eq(&other)),
        );

        methods.add_meta_method(
            mlua::MetaMethod::Lt,
            |_, this, other: mlua::UserDataRef<Self>| {
                Ok(matches!(this.cmp(&other), Ordering::Less))
            },
        );

        methods.add_meta_method(
            mlua::MetaMethod::Le,
            |_, this, other: mlua::UserDataRef<Self>| {
                Ok(matches!(this.cmp(&other), Ordering::Less | Ordering::Equal))
            },
        );
    }
}
