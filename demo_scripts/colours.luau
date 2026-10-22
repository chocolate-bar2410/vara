--!strict
local colour = require("@vara/colour")
local io = require("@vara/io")

local function colour_bar(... : Colour4)
    for _,colour in {...} do
        io.Write(colour:ToAnsi() .. "██\x1b[0m")
    end

    io.Write("\n")
end


local A = colour.FromOKLCH(0.8264, 0.1109, 65.7):Brighten(0.1):HdrTonemap()
print(A)

colour_bar(A,A:Triadic())
colour_bar(A,A:SplitComplement())
