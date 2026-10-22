--!strict
local colour = require("@vara/colour")

local A = colour.New(0.5,0.5,0.5)

print(A:Luminance())

print(A:Greyscale())