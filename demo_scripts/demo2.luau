local serde = require("@vara/serde")

local message = serde.encode("JSON", { hello = "world", testing = 123 })
print(message)
