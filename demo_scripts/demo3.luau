--!strict
local system = require("@vara/system")
local pretty_table = require("./pretty_table")

print("os:",system.os)
print("arch:",system.arch)
print("family:",system.family)
print("cwd:",system.cwd)
print("temp:",system.temp)
print("endian:",system.endian)

print("distro:",system.distro)
print("available cores:",system.processorCount())

-- print("language preferences:",pretty_table(system.languagePreferences :: {unknown}))