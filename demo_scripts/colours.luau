local colour = require("@vara/colour")
local io = require("@vara/io")

local A = colour.New(0.5,0.5,0.5)
local B = colour.FromSRGBA(0.5,0.5,0.5)

io.Write(A:ToAnsi() .. "████" .. "\x1b[0m")
io.Write(B:ToAnsi() .. "████" .. "\x1b[0m\n")

io.Flush()

print(("%.2f %.2f %.2f %.2f"):format(A:ToSRGBA()))