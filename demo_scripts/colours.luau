--!strict
local colour = require("@vara/colour")

local A = colour.New(0.5,0.5,0.5)

print(("%.2f %.2f %.2f %.2f"):format(A:ToOKLCH()))