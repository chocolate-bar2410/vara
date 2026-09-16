local fs = require("@vara/fs")
local serde = require("@vara/serde")

local load = require("./demo3")
local message = serde.encode("YAML", load)
print(message)

local data: { hello: string, testing: number } = serde.decode("YAML", message)
print(data.globals.hello)
print(data.globals.testing)

fs.write("testing.yaml", message)
