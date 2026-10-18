--!strict
local io = require("@vara/io")
local terminal = require("@vara/terminal")

local style = terminal.Style
local colour = terminal.Colour

terminal.EnterAltScreen()
terminal.EnterRawMode()
terminal.HideCursor()

terminal.MoveCursor(1,2)
io.Write(`{style.Italic}{colour.BrightCyan}hi guys{style.Reset}`)

terminal.MoveCursor(1,1)
io.Write(("-"):rep(18))

terminal.MoveCursor(1,3)
io.Write(("-"):rep(18))
io.Flush()

repeat
until io.Read(4) == "\r"

terminal.ShowCursor()
terminal.ExitRawMode()
terminal.ExitAltScreen()