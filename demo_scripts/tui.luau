local io = require("@vara/io")
local terminal = require("@vara/terminal")

terminal.Clear()
terminal.HideCursor(false)

terminal.MoveCursor(10,5)
io.Write("hi guys\n")

io.ReadLine(1)
terminal.ShowCursor()