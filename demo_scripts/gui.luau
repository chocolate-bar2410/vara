--!strict
local chrono = require("@vara/chrono")
local system = require("@vara/system")
local ui = require("@vara/ui")

local context = ui.CreateContext()

local window_A = context:CreateWindow("こんにちは")

window_A.OnClose = function()
    local should_exit = system.YesNoDialogue("Exiting app", "Are you sure you want to exit?")
    print(should_exit)
    return should_exit
end

context:Run()