--!strict
local colour = require("@vara/colour")

const module = {}

module.colour_bar = function(pallete : {Colour4})
    local result = ""

    for _,colour in pallete do
        result ..= colour:ToAnsi() .. "████\x1b[0m"
    end

    return result .. "\n" .. result
end

module.generate_pallete = function() : {Colour4}
    local hue = math.random(0,360)
    local chroma = math.map(math.random(),0,1,0.1,0.21)
    local lightness = math.map(math.random(),0,1,0.4,0.6)

    local light_inc = math.map(math.random(),0,1,0.02,0.1)
    local hue_inc = math.random(7,10)

    local pallete = {}

    local seed = colour.FromOKLCH(lightness, chroma,hue)

    table.insert(pallete,seed)

    for i = 1,9 do
        hue = (hue + hue_inc) % 360
        lightness += light_inc

        pallete[#pallete + 1] = colour.FromOKLCH(lightness, chroma, hue)
    end

    for i = 1,#pallete do
        pallete[i]:HdrTonemap()
    end

    return pallete
end

return module
