--!strict
local chrono = require("@vara/chrono")
local system = require("@vara/system")
local task = require("@vara/task")
local ui = require("@vara/ui")

local context = ui.CreateContext()

local window_A = context:CreateWindow("こんにちは")

local should_close = false

local subroutine = coroutine.create(function()
    while not should_close do
        print("testing")
        task.Sleep(0.1)
    end
end)


window_A.OnClose = function()
    local should_exit = system.YesNoDialogue("Exiting app", "Are you sure you want to exit?")
    print(should_exit)
    return should_exit
end

coroutine.resume(subroutine)
context:Run()

should_close = true