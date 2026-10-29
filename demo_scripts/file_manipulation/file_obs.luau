--!strict
local fs = require("@vara/fs")
local io = require("@vara/io")

print("started watching file")

local observer = fs.Observer("./demo_scripts/images")

io.ReadLine()

observer:Disconnect()

print("stopped watching file")

