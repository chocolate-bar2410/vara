--!strict
local io = require("@vara/io")
local terminal = require("@vara/terminal")

local style = terminal.Style
local colour = terminal.Colour

terminal.Clear()
terminal.HideCursor()

terminal.MoveCursor(1,2)
io.Write(`{style.Italic}{colour.BrightCyan}hi guys{style.Reset}`)

terminal.MoveCursor(1,1)
io.Write(("-"):rep(18))

terminal.MoveCursor(1,3)
io.Write(("-"):rep(18))
io.Flush()

io.ReadLine(1)
terminal.ShowCursor()
terminal.Clear()