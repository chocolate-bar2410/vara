local task = require("@vara/task")

local loop1 = coroutine.create(function()
    while true do
        print("task 1")
        task.Sleep()
    end
end)

coroutine.resume(loop1)

while true do
    print("task 2")
    task.Sleep()
end