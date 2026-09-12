local fs = require("@vara/fs")
local serde = require("@vara/serde")

local message = serde.encode("TOML", { hello = "world", testing = 123 })
print(message)

local data: { hello: string, testing: number } = serde.decode("TOML", message)
print(data.hello)
print(data.testing)

fs.write("testing.toml", message)
