--!strict
local fs = require("@vara/fs")
local io = require("@vara/io")
local path = require("@vara/path")
local system = require("@vara/system")

print("started watching file")

local downloads = fs.Directory(path.Join(system.Home,"Downloads"))

local observer = fs.Observer(downloads)

local debounce = os.clock()

observer.OnCreated = function(paths)
    if debounce > os.clock() then return end
    debounce = os.clock() + 0.1

    print("new files added")

    for i,v in paths do
        print(`[{i}]: {path.Absolute(v)}`)
    end
end

repeat
until io.ReadAsync(1)

observer:Disconnect()

print("stopped watching file")

