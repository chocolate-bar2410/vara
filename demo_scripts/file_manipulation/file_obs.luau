--!strict
local fs = require("@vara/fs")
local io = require("@vara/io")
local path = require("@vara/path")

print("started watching file")

local observer = fs.Observer("./demo_scripts/images")

local debounce = os.clock()

observer.OnModified = function(paths)
    if debounce > os.clock() then return end
    debounce = os.clock() + 0.1

    print("new files changed")

    for i,v in paths do
        print(`[{i}]: {path.Absolute(v)}`)
    end
end

repeat
until io.ReadAsync(1)

observer:Disconnect()

print("stopped watching file")

