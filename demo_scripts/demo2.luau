local fs = require("@vara/fs")
local serde = require("@vara/serde")

local message = serde.encode("YAML", { globals = { hello = "world", testing = 123 } })
print(message)

local data: { hello: string, testing: number } = serde.decode("YAML", message)
print(data.globals.hello)
print(data.globals.testing)

fs.write("testing.yaml", message)
