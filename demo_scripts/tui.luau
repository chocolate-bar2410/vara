local io = require("@vara/io")
local terminal = require("@vara/terminal")

terminal.Clear()
terminal.HideCursor(false)

io.ReadLine()

terminal.ShowCursor()