local colour = require("@vara/colour")

local A = colour.FromHex("#FF00FF7F")

print(("%.2f %.2f %.2f %.2f"):format(A:ToSRGBA()))