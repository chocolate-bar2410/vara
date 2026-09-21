--!strict
local system = require("@vara/system")

print("os:",system.os)
print("arch:",system.arch)
print("family:",system.family)
print("cwd:",system.cwd)
print("temp:",system.temp)
print("endian:",system.endian)