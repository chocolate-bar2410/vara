--!strict
local colour = require("@vara/colour")

local A = colour.FromOKLCH(0.8264, 0.1109, 65.7)

print(A:ToAnsi() .. "##" .. A:Complementary():ToAnsi() .. "##\x1b[0m")