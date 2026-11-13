--!strict
local fs = require("@vara/fs")
local system = require("@vara/system")
local task = require("@vara/task")

local dir = fs.Directory(".")

local file = system.PickFileDialogue("choose a file", dir, {})
print(file)