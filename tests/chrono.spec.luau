local chrono = require("@vara/chrono")
local system = require("@vara/system")

local timestampA = os.time();

local date = os.date("!*t",timestampA)
local varaDate = chrono.fromUnix(timestampA)

assert(date.year == varaDate.year,`{date.year} ≠ {varaDate.year}`)
assert(date.month == varaDate.month,`{date.month} ≠ {varaDate.month}`)
assert(date.day == varaDate.day,`{date.day} ≠ {varaDate.day}`)
assert(date.hour == varaDate.hour,`{date.hour} ≠ {varaDate.hour}`)
assert(date.min == varaDate.minute,`{date.min} ≠ {varaDate.minute}`)
assert(date.sec == varaDate.second,`{date.sec} ≠ {varaDate.second}`)

local dateA = chrono.fromUnix(0)
local dateB = chrono.fromUnix(-1)

assert(dateA > dateB)

local dateC = chrono.fromUniversal({
    year = 1970,
    month = 1,
    day = 1,
    hour = 0,
    minute = 0,
    second = 0
})

assert(dateC == chrono.fromUnix(0),"fromUniversal conversion failed")

-- haven't implemented local timezones for unix systems
if system.os ~= "windows" then return end

local dateD = chrono.fromLocal({
    year = 1970,
    month = 1,
    day = 1,
    hour = 0,
    minute = 0,
    second = 0
})

local dateE = chrono.fromUnix(0)
dateE.timeOffset = chrono.timezoneOffset();

assert(dateD == dateE,"fromLocal conversion failed")

local timestampB = os.time();

local timezoneDate = os.date("*t",timestampB)
local timezonevaraDate = chrono.fromUnix(timestampB)
timezonevaraDate.timeOffset = chrono.timezoneOffset()

assert(timezoneDate.year == timezonevaraDate.year,`{timezoneDate.year} ≠ {timezonevaraDate.year}`)
assert(timezoneDate.month == timezonevaraDate.month,`{timezoneDate.month} ≠ {timezonevaraDate.month}`)
assert(timezoneDate.day == timezonevaraDate.day,`{timezoneDate.day} ≠ {timezonevaraDate.day}`)
assert(timezoneDate.hour == timezonevaraDate.hour,`{timezoneDate.hour} ≠ {timezonevaraDate.hour}`)
assert(timezoneDate.min == timezonevaraDate.minute,`{timezoneDate.min} ≠ {timezonevaraDate.minute}`)
assert(timezoneDate.sec == timezonevaraDate.second,`{timezoneDate.sec} ≠ {timezonevaraDate.second}`)
