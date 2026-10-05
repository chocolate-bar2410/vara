--[=[
	@class chrono

	Built in library for managing time.

	### example code
	```luau
	local chrono = require("@vara/chrono")

    local timestampC = chrono.now()

    print(`ISO8601:   {timestampC:toISO8601()}`)
    print(`RFC2822:   {timestampC:toRFC2822()}`)
    print(`humanised: {timestampC:humanise()}`)
    print(`中国:      {timestampC:toZH()}`)
    print(`Apache:   {timestampC:toApacheLog()}`)
	```
]=]
const module = {}

type DateTimeParameters = {
    year : number,
    month : number,
    day : number,

    hour : number,
    minute : number,
    second : number,
    millisecond : number?,
}

--[=[
	@within chrono
	@method now
	@return DateTime

	Gets the current date and time according adjusted to the local timezone.
]=]
function module.now() : DateTime
    return nil :: unknown
end

--[=[
	@within chrono
	@method fromUnix
	@param unixTime The ammount of seconds since january 1st 1970 at UTC+00.
	@return DateTime

	Converts a unix timestamp into a DateTime.
]=]
function module.fromUnix(unixTime : number) : DateTime
    return nil :: unknown
end

--[=[
	@within chrono
	@method fromUniversal
	@param params The specified date and time as table parameters.
	@return DateTime

	Constructs a DateTime using a table of parameters at UTC+00.
]=]
function module.fromUniversal(params : DateTimeParameters) : DateTime
    return nil :: unknown
end


--[=[
	@within chrono
	@method fromLocal
	@param params The specified date and time as table parameters.
	@return DateTime

	Constructs a DateTime using a table of parameters in the local timezone.
]=]
function module.fromLocal(params : DateTimeParameters) : DateTime
    return nil :: unknown
end

--[=[
	@within chrono
	@method timezoneOffset
	@return DateTime

	returns the current timezone offset in hours.
]=]
function module.timezoneOffset() : number
    return nil :: unknown
end

return module
