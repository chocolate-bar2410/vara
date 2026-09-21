--[=[
    @class process
    Built in class for accessing system info.
]=]
local module = {}


--[=[
    @within process
    @prop os
    @tag read_only

    The operating system being used.
]=]
module.os = (nil :: unknown) :: "linux" | "macos" | "windows"
--[=[
    @within process
    @prop arch
    @tag read_only

    The architecture of the processor thats currently being used.
]=]
module.arch = (nil :: unknown) :: "x86_64" | "aarch64"
--[=[
    @within process
    @prop family
    @tag read_only

    The family of operating system being used.
]=]
module.family = (nil :: unknown) :: "unix" | "windows"
--[=[
    @within process
    @prop cwd
    @tag read_only

    The current working directory.
]=]
module.cwd = (nil :: unknown) :: string
--[=[
    @within process
    @prop temp
    @tag read_only

    The temp directory.
]=]
module.temp = (nil :: unknown) :: string
--[=[
    @within process
    @prop endian
    @tag read_only

    The endianess of the processor being used.
]=]
module.endian = (nil :: unknown) :: "big" | "little"
--[=[
    @within process
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
    @within process
    @prop distro
    @tag read_only

    What operating system distribution the system is using.
]=]
module.distro = (nil :: unknown) :: string

--[=[
    @within process
    @prop device
    @tag read_only

    The name of the device currently being used.
]=]
module.device = (nil :: unknown) :: string

--[=[
    @within process
    @method processorCount

    @returns The number of logical CPUs available on the system.
]=]
module.processorCount = (nil :: unknown) :: () -> number

return module