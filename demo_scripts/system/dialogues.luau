--!strict
local fs = require("@vara/fs")
local system = require("@vara/system")
local task = require("@vara/task")

local dir = fs.Directory("demo_scripts/images")

local file = system.SaveFileDialogue("choose a file", dir, {})
print(file)