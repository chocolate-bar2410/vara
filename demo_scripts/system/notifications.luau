local io = require("@vara/io")
local system = require("@vara/system")

print("\x1b[1mdesktop notification writer\x1b[0m")
io.Write("enter summary: ")
io.Flush()
local summary = io.ReadLine()

io.Write("enter description: ")
io.Flush()
local description = io.ReadLine()

system.Notify(summary, description, "notifier", 1000)