--!strict
local system = require("@vara/system")
local task = require("@vara/task")

if system.OkDialogue("Option", "This is an option",true) then 
    print("success")
else
    print("cancelled")
end
