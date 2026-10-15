local task = require("@vara/task")

local loop1 = function()
    for i = 1,20 do
        print("task 1")
        task.Sleep(0.1)
    end
    
    return "task 1"
end

local loop2 = function()
    for i = 1,10 do
        print("task 2")
        task.Sleep(0.1)
    end

    return "task 2"
end

local winner = task.Race(loop1,loop2)

print(`{winner} completed first`)