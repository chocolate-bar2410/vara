--!strict
local system = require("@vara/system")
local task = require("@vara/task")

local result = system.YesNoDialogue("Option", "This is an option",true) 

if result == true then 
    print("yes")
elseif result == false then
    print("no")
else
    print("cancelled")
end
