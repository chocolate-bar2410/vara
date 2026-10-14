local net = require("@vara/net")
local serde = require("@vara/serde")
local pretty_table = require("./pretty_table")

local response = net.HttpClient():GetAsync("https://jsonplaceholder.typicode.com/posts")

local users = serde.Decode("JSON",buffer.tostring(response.Body))

for _,user in users do
    print(`\x1b[1m{user.title}\x1b[0m`)
    print(`\x1b[1mUser\x1b[0m: \x1b[34m{user.userId}\x1b[0m`)
    print(`\x1b[1mID\x1b[0m:   \x1b[34m{user.id}\x1b[0m`)
    print(`\x1b[2m{user.body}\x1b[0m\n`)
end
