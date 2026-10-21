--[=[
    @class colourLib
    Built in library for handling colours.

]=]
local module = {}

--[=[
    @within colourLib
    @method new
    @param red
    @param green
    @param blue
    @param alpha The transparency of the colour.
    @return colour4 value.

    Creates a new Colour4.
]=]
module.New = function(red: number,green: number,blue: number,alpha: number?) : Colour4
    return nil :: unknown
end

--[=[
    @within colourLib
    @method new
    @param red
    @param green
    @param blue
    @param alpha The transparency of the colour.
    @return colour4 value.

    Creates a new Colour4 from the given srgba values.
]=]
module.FromSRGBA = function(red: number,green: number,blue: number,alpha: number?) : Colour4
    return nil :: unknown
end

return module