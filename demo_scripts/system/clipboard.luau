--!strict
local system = require("@vara/system")

local clipboard = system.Clipboard()
local text = clipboard:GetText()

print(text)