--!strict
local fs = require("@vara/fs")
local io = require("@vara/io")

print("started watching file")

repeat
until io.ReadAsync(1) == string.char(13)

print("stopped watching file")

