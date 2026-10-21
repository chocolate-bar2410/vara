local colour = require("@vara/colour")

local my_colour = colour.New(1, 0, 1)

print(my_colour:ToAnsi() .. "#" .. "\x1b[0m")