--[=[
	@class chrono

	Built in library for managing time.

	### example code
	```luau
	local chrono = require("@vara/chrono")

    local timestamp = chrono.Now()

    print(`ISO8601:   {timestamp:ToISO8601()}`)
    print(`RFC2822:   {timestamp:ToRFC2822()}`)
    print(`humanised: {timestamp:Humanise()}`)
    print(`中国:      {timestamp:ToZH()}`)
    print(`Apache:   {timestamp:ToApacheLog()}`)
	```
]=]
const module = {}

type DateTimeParameters = {
    Year : number,
    Month : number,
    Day : number,

    Hour : number,
    Minute : number,
    Second : number,
    Millisecond : number?,
}

--[=[
	@within chrono
	@method Now
	@return DateTime

	Gets the current date and time according adjusted to the local timezone.
]=]
function module.Now() : DateTime
    return nil :: unknown
end

--[=[
	@within chrono
	@method FromUnix
	@param unixTime The ammount of seconds since january 1st 1970 at UTC+00.
	@return DateTime

	Converts a unix timestamp into a DateTime.
]=]
function module.FromUnix(unixTime : number) : DateTime
    return nil :: unknown
end

--[=[
	@within chrono
	@method FromUniversal
	@param params The specified date and time as table parameters.
	@return DateTime

	Constructs a DateTime using a table of parameters at UTC+00.
]=]
function module.FromUniversal(params : DateTimeParameters) : DateTime
    return nil :: unknown
end


--[=[
	@within chrono
	@method FromLocal
	@param params The specified date and time as table parameters.
	@return DateTime

	Constructs a DateTime using a table of parameters in the local timezone.
]=]
function module.FromLocal(params : DateTimeParameters) : DateTime
    return nil :: unknown
end

--[=[
	@within chrono
	@method TimezoneOffset
	@return DateTime

	returns the current timezone offset in hours.
]=]
function module.TimezoneOffset() : number
    return nil :: unknown
end

return module
