--!strict
local colour = require("@vara/colour")

local A = colour.FromHSVA(320, 0.45, 0.85)

print(("%.2f %.2f %.2f %.2f"):format(A:ToHSVA()))