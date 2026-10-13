local net = require("@vara/net")
local serde = require("@vara/serde")
local pretty_table = require("./pretty_table")

local response = net.HttpClient():GetAsync("https://httpbin.org/get")

print(pretty_table(serde.Decode("JSON",buffer.tostring(response.Body))))