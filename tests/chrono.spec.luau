local chrono = require("@vara/chrono")
local system = require("@vara/system")
local unit = require("@vara/test/unit")

local module = {}

function module.UnixConversion()
    local timestamp = os.time()

    local date = os.date("!*t",timestamp)
    local varaDate = chrono.FromUnix(timestamp)

    unit.AssertEquals(date.year,varaDate.Year)
    unit.AssertEquals(date.month,varaDate.Month)
    unit.AssertEquals(date.day,varaDate.Day)
    unit.AssertEquals(date.hour,varaDate.Hour)
    unit.AssertEquals(date.min,varaDate.Minute)
    unit.AssertEquals(date.sec,varaDate.Second)
end

function module.CompareDateTimes()
    local dateA = chrono.FromUnix(0)
    local dateB = chrono.FromUnix(-1)

    unit.AssertTrue(dateA > dateB)
end

function module.FromUniversal()
    local dateC = chrono.FromUniversal({
        Year = 1970,
        Month = 1,
        Day = 1,
        Hour = 0,
        Minute = 0,
        Second = 0
    })

    unit.AssertEquals(dateC,chrono.FromUnix(0))
end

-- haven't implemented local timezones for unix systems
function module.FromLocal()
    if system.Os ~= "windows" then return end

    local dateA = chrono.FromLocal({
        Year = 1970,
        Month = 1,
        Day = 1,
        Hour = 0,
        Minute = 0,
        Second = 0
    })

    local dateB = chrono.FromUnix(0)
    dateB.TimeOffset = chrono.TimezoneOffset()

    unit.AssertEquals(dateA, dateB)
end

function module.UnixConversionTimezone()
    if system.Os ~= "windows" then return end

    local timestamp = os.time()

    local timezoneDate = os.date("*t",timestamp)
    local timezonevaraDate = chrono.FromUnix(timestamp)
    timezonevaraDate.TimeOffset = chrono.TimezoneOffset()

    unit.AssertEquals(timezoneDate.year, timezonevaraDate.Year)
    unit.AssertEquals(timezoneDate.month, timezonevaraDate.Month)
    unit.AssertEquals(timezoneDate.day, timezonevaraDate.Day)
    unit.AssertEquals(timezoneDate.hour, timezonevaraDate.Hour)
    unit.AssertEquals(timezoneDate.min, timezonevaraDate.Minute)
    unit.AssertEquals(timezoneDate.sec, timezonevaraDate.Second)
end

return module

