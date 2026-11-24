--!strict
local system = require("@vara/system")
local pretty_table = require("./pretty_table")

print("os:",system.Os)
print("arch:",system.Arch)
print("family:",system.Family)
print("endian:",system.Endian)

print("cwd:",system.Cwd)
print("temp:",system.Temp)
print("home:",system.Home)

print("distro:",system.Distro)
print("cores:",system.ProcessorCount())

print("desktop:",system.DesktopEnvironment)
print("processID:",system.ProcessID)

print("physical memory usage:",system.PhysicalMemoryUsage())
print("virtual memory usage:",system.VirtualMemoryUsage())

print("language preferences:",pretty_table(system.LanguagePreferences :: {unknown}))
print("environment variables:",pretty_table(system.Env))
print("env TESTING:",system.Env.TESTING)

print("args:",pretty_table(system.Args))
