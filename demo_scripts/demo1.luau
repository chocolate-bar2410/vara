--!strict
local fs = require("@vara/fs")
local serde = require("@vara/serde")

local load = require("./democonfig")
local message = serde.Encode("YAML", load)
print(message)

local data: typeof(load) = serde.Decode("YAML", message) :: typeof(load)
print(data.globals.hello)
print(data.globals.testing)

fs.write("testing.yaml", message)
