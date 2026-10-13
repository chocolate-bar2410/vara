local net = require("@vara/net")
local pretty_table = require("./pretty_table")

local response = net.HttpClient():GetAsync("https://example.com")

print(response)