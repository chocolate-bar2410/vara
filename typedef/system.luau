--[=[
	@class system
    Built in class for accessing system info.
]=]
local module = {}


--[=[
	@within system
	@prop Os
	@tag read_only

    The operating system being used.
]=]
module.Os = (nil :: unknown) :: "linux" | "macos" | "windows"
--[=[
	@within system
	@prop Arch
	@tag read_only

    The architecture of the processor thats currently being used.
]=]
module.Arch = (nil :: unknown) :: "x86_64" | "aarch64"
--[=[
	@within system
	@prop Family
	@tag read_only

    The family of operating system being used.
]=]
module.Family = (nil :: unknown) :: "unix" | "windows"
--[=[
	@within system
	@prop Cwd
	@tag read_only

    The current working directory.
]=]
module.Cwd = (nil :: unknown) :: string
--[=[
	@within system
	@prop Temp
	@tag read_only

    The temp directory.
]=]
module.Temp = (nil :: unknown) :: string
--[=[
	@within system
	@prop Home
	@tag read_only

    The home directory.
]=]
module.Home = (nil :: unknown) :: string
--[=[
	@within system
	@prop Endian
	@tag read_only

    The endianess of the processor being used.
]=]
module.Endian = (nil :: unknown) :: "big" | "little"
--[=[
	@within system
	@prop LanguagePreferences
	@tag read_only

    Language preferences set on the user's system.
]=]
module.LanguagePreferences = (nil :: unknown) :: {
    Time : {string},
    Monetary : {string},
    Collation : {string},
    Message : {string},
    Numeric : {string},
}

--[=[
	@within system
	@prop Distro
	@tag read_only

    What operating system distribution the system is using.
]=]
module.Distro = (nil :: unknown) :: string

--[=[
	@within system
	@prop Device
	@tag read_only

    The name of the device currently being used.
]=]
module.Device = (nil :: unknown) :: string

--[=[
	@within system
	@prop ProcessID
	@tag read_only

    The PID of the current process.
]=]
module.ProcessID = (nil :: unknown) :: number

--[=[
	@within system
	@prop DesktopEnvironment
	@tag read_only

    The desktop environment the system is using.
]=]
module.DesktopEnvironment = (nil :: unknown) :: number

--[=[
	@within system
	@prop Env
	@tag read_only

    The environment variables declared by the system and a optional .env file.
]=]
module.Env = (nil :: unknown) :: {[string] : string}

--[=[
	@within system
	@prop Args
	@tag read_only

    Command line arguments passed into the program.
]=]
module.Args = (nil :: unknown) :: {[number] : string}

--[=[
	@within system
	@method ProcessorCount

	@return The number of logical CPUs available on the system.
]=]
function module.ProcessorCount() : number
    return nil :: unknown
end

--[=[
	@within system
	@method PhysicalMemoryUsage

	@return The ammount of physical memory being used up by the process.
]=]
function module.PhysicalMemoryUsage() : number
    return nil :: unknown
end

--[=[
	@within system
	@method VirtualMemoryUsage

	@return The ammount of virtual memory being used up by the process.
]=]
function module.VirtualMemoryUsage() : number
    return nil :: unknown
end

return module