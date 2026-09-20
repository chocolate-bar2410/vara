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
	@class DateTime

    An Object representing a date and time.
    code example:
    ```luau
    local chrono = require("@vara/chrono")

    local timestamp = chrono.fromUniversal({
        year = 2025,
        month = 9,
        day = 10,

        hour = 18,
        minute = 20,
        second = 0,
        millisecond = 0,
    })

    print(timestamp)
    ```
]=]
export type DateTime = setmetatable<{
    --- The UTC offset in hours.
    timeOffset : number,
    year : number,
    month : number,
    day : number,
    hour : number,
    minute : number,
    second : number,
    millisecond : number,

    --[=[
		@within DateTime
		@method toISO8601
        Formats the datetime according to the ISO8601 standard.
    ]=]
    toISO8601 : (self : DateTime) -> string,

    --[=[
		@within DateTime
		@method toRFC2822
        Formats the datetime according to the RFC2822 standard.
    ]=]
    toRFC2822 : (self : DateTime) -> string,

    --[=[
		@within DateTime
		@method humanise
        Formats the datetime so thats its human readable.
    ]=]
    humanise : (self : DateTime) -> string,

    --[=[
		@within DateTime
		@method toZH
        Formats the datetime so that its human readable in mandarin.
    ]=]
    toZH : (self : DateTime) -> string,

    --[=[
		@within DateTime
		@method toApacheLog
        Formats the datetime so as an Apache Log timestamp.
    ]=]
    toApacheLog : (self : DateTime) -> string,
},{
    __tostring : (self : DateTime) -> string,
    __eq : (self : DateTime,other : DateTime) -> boolean,
    __le : (self : DateTime,other : DateTime) -> boolean,
    __lt : (self : DateTime,other : DateTime) -> boolean,
}>

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

return module
