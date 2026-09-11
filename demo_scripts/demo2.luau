local serde = require("@vara/serde")

local message = serde.encode("JSON", { hello = "world", testing = 123 })
print(message)

local data: { hello: string, testing: number } = serde.decode("JSON", message)
print(data.hello)
print(data.testing)
