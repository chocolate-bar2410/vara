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
    @method FromSRGBA
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

--[=[
    @within colourLib
    @method FromHex
    @param hex_string a hexidecimal value that represents a srgba value.
    @return colour4 value.

    Creates a new Colour4 from a hex string.
]=]
module.FromHex = function(hex_string: string) : Colour4
    return nil :: unknown
end

--[=[
    @within colourLib
    @method FromHSVA
    @param hue Angle that represents the colour on the colour wheel in degrees.
    @param saturation How strong the colour is from 0 - 1.
    @param value How bright the colour is from 0 - 1. 
    @param alpha The transparency of the colour.
    @return colour4 value.

    Creates a new Colour4 from a hue, saturation and value.
]=]
module.FromHSVA = function(hue: number,saturation: number,value: number,alpha: number?) : Colour4
    return nil :: unknown
end

return module