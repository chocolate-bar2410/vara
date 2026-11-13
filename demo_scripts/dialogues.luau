--!strict
local system = require("@vara/system")
local task = require("@vara/task")

task.Spawn(function() system.InfoDialogue("Testing", "This is a system popup") end)
task.Spawn(function() system.InfoDialogue("Warning", "This is a warning","WARN") end)
system.InfoDialogue("Error", "This is an error","ERROR") 