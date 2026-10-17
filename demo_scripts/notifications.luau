local system = require("@vara/system")

local summary = "hi"
local body = "testing 123"
local name = "vara"
local timeout = 10

system.Notify(summary, body, name, timeout)