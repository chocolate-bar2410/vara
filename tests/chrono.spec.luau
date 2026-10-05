local chrono = require("@vara/chrono")
local system = require("@vara/system")

local timestampA = os.time();

local date = os.date("!*t",timestampA)
local varaDate = chrono.FromUnix(timestampA)

assert(date.year == varaDate.Year,`{date.year} ≠ {varaDate.Year}`)
assert(date.month == varaDate.Month,`{date.month} ≠ {varaDate.Month}`)
assert(date.day == varaDate.Day,`{date.day} ≠ {varaDate.Day}`)
assert(date.hour == varaDate.Hour,`{date.hour} ≠ {varaDate.Hour}`)
assert(date.min == varaDate.Minute,`{date.min} ≠ {varaDate.Minute}`)
assert(date.sec == varaDate.Second,`{date.sec} ≠ {varaDate.Second}`)

local dateA = chrono.FromUnix(0)
local dateB = chrono.FromUnix(-1)

assert(dateA > dateB)

local dateC = chrono.FromUniversal({
    Year = 1970,
    Month = 1,
    Day = 1,
    Hour = 0,
    Minute = 0,
    Second = 0
})

assert(dateC == chrono.FromUnix(0),"fromUniversal conversion failed")

-- haven't implemented local timezones for unix systems
if system.Os ~= "windows" then return end

local dateD = chrono.FromLocal({
    Year = 1970,
    Month = 1,
    Day = 1,
    Hour = 0,
    Minute = 0,
    Second = 0
})

local dateE = chrono.FromUnix(0)
dateE.TimeOffset = chrono.TimezoneOffset();

assert(dateD == dateE,"fromLocal conversion failed")

local timestampB = os.time();

local timezoneDate = os.date("*t",timestampB)
local timezonevaraDate = chrono.FromUnix(timestampB)
timezonevaraDate.TimeOffset = chrono.TimezoneOffset()

assert(timezoneDate.year == timezonevaraDate.Year,`{timezoneDate.year} ≠ {timezonevaraDate.Year}`)
assert(timezoneDate.month == timezonevaraDate.Month,`{timezoneDate.month} ≠ {timezonevaraDate.Month}`)
assert(timezoneDate.day == timezonevaraDate.Day,`{timezoneDate.day} ≠ {timezonevaraDate.Day}`)
assert(timezoneDate.hour == timezonevaraDate.Hour,`{timezoneDate.hour} ≠ {timezonevaraDate.Hour}`)
assert(timezoneDate.min == timezonevaraDate.Minute,`{timezoneDate.min} ≠ {timezonevaraDate.Minute}`)
assert(timezoneDate.sec == timezonevaraDate.Second,`{timezoneDate.sec} ≠ {timezonevaraDate.Second}`)
