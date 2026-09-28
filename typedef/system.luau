--[=[
	@class system
    Built in class for accessing system info.
]=]
local module = {}


--[=[
	@within system
	@prop os
	@tag read_only

    The operating system being used.
]=]
module.os = (nil :: unknown) :: "linux" | "macos" | "windows"
--[=[
	@within system
	@prop arch
	@tag read_only

    The architecture of the processor thats currently being used.
]=]
module.arch = (nil :: unknown) :: "x86_64" | "aarch64"
--[=[
	@within system
	@prop family
	@tag read_only

    The family of operating system being used.
]=]
module.family = (nil :: unknown) :: "unix" | "windows"
--[=[
	@within system
	@prop cwd
	@tag read_only

    The current working directory.
]=]
module.cwd = (nil :: unknown) :: string
--[=[
	@within system
	@prop temp
	@tag read_only

    The temp directory.
]=]
module.temp = (nil :: unknown) :: string
--[=[
	@within system
	@prop home
	@tag read_only

    The home directory.
]=]
module.home = (nil :: unknown) :: string
--[=[
	@within system
	@prop endian
	@tag read_only

    The endianess of the processor being used.
]=]
module.endian = (nil :: unknown) :: "big" | "little"
--[=[
	@within system
	@prop languagePreferences
	@tag read_only

    Language preferences set on the user's system.
]=]
module.languagePreferences = (nil :: unknown) :: {
    time : {string},
    monetary : {string},
    collation : {string},
    message : {string},
    numeric : {string},
}

--[=[
	@within system
	@prop distro
	@tag read_only

    What operating system distribution the system is using.
]=]
module.distro = (nil :: unknown) :: string

--[=[
	@within system
	@prop device
	@tag read_only

    The name of the device currently being used.
]=]
module.device = (nil :: unknown) :: string

--[=[
	@within system
	@prop processID
	@tag read_only

    The PID of the current process.
]=]
module.processID = (nil :: unknown) :: number

--[=[
	@within system
	@prop desktopEnvironment
	@tag read_only

    The desktop environment the system is using.
]=]
module.desktopEnvironment = (nil :: unknown) :: number

--[=[
	@within system
	@prop env
	@tag read_only

    The environment variables declared by the system and a optional .env file.
]=]
module.env = (nil :: unknown) :: {[string] : string}

--[=[
	@within system
	@prop args
	@tag read_only

    Command line arguments passed into the program.
]=]
module.args = (nil :: unknown) :: {[string] : string}

--[=[
	@within system
	@method processorCount

	@return The number of logical CPUs available on the system.
]=]
function module.processorCount() : number
    return nil :: unknown
end

--[=[
	@within system
	@method physicalMemoryUsage

	@return The ammount of physical memory being used up by the process.
]=]
function module.physicalMemoryUsage() : number
    return nil :: unknown
end

--[=[
	@within system
	@method virtualMemoryUsage

	@return The ammount of virtual memory being used up by the process.
]=]
function module.virtualMemoryUsage() : number
    return nil :: unknown
end

return module