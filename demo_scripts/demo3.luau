--!strict
local system = require("@vara/system")
local pretty_table = require("./pretty_table")

print("os:",system.os)
print("arch:",system.arch)
print("family:",system.family)
print("endian:",system.endian)

print("cwd:",system.cwd)
print("temp:",system.temp)
print("home:",system.home)

print("distro:",system.distro)
print("cores:",system.processorCount())

print("desktop:",system.desktopEnvironment)
print("processID:",system.processID)

print("physical memory usage:",system.physicalMemoryUsage())
print("virtual memory usage:",system.virtualMemoryUsage())

print("language preferences:",pretty_table(system.languagePreferences :: {unknown}))
print("environment variables:",pretty_table(system.env))
print("env TESTING:",system.env.TESTING)

print("args:",pretty_table(system.args))
