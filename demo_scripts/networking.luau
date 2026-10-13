local net = require("@vara/net")

local result,status = net.HttpClient().GetAsync("http://example.com")

print(buffer.tostring(result))