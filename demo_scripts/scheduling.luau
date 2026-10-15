local task = require("@vara/task")

local loop1 = function()
    for i = 1,20 do
        print("task 1",i)
        task.Sleep(0.1)
    end
    print("finished")
end

local loop2 = function()
    for i = 1,10 do
        print("task 2")
        task.Sleep(0.1)
    end
end

task.All(loop1,loop2)