--!strict
local colour = require("@vara/colour")
local media = require("@vara/media")
local unit = require("@vara/test/unit")
const module = {}

local width = 16
local height = 16

local image = media.Image(buffer.create(width * height * 4 * 4)):Dimensions(width, height)

for y = 0,height - 1  do
    for x = 0,width  - 1 do
        image:SetPixel(x, y, colour.New(1,1,1))
    end
end

local get_pixel = function(x: number,y: number)
    local index = (y * width + x) * 16 

    local red = buffer.readf32(image.Data, index)
    local green = buffer.readf32(image.Data, index + 4)
    local blue = buffer.readf32(image.Data, index + 8)
    local alpha = buffer.readf32(image.Data, index + 12)

    return red,green,blue,alpha
end

local set_pixel = function(x: number,y: number,data: Colour4)
    local index = (y * width + x) * 16 

    buffer.writef32(image.Data, index,data.Red)
    buffer.writef32(image.Data, index + 4,data.Green)
    buffer.writef32(image.Data, index + 8,data.Blue)
    buffer.writef32(image.Data, index + 12,data.Alpha)
end

module["Set pixel"] = function()
    image:SetPixel(0, 0, colour.New(1,0,1))
    image:SetPixel(15, 15, colour.New(0,1,1))
    image:SetPixel(8, 7, colour.New(1,1,0))

    local red,green,blue,alpha = get_pixel(0,0)

    unit.AssertEquals(1, red)
    unit.AssertEquals(0, green)
    unit.AssertEquals(1, blue)
    unit.AssertEquals(1, alpha)

    red,green,blue,alpha = get_pixel(15,15)

    unit.AssertEquals(0, red)
    unit.AssertEquals(1, green)
    unit.AssertEquals(1, blue)
    unit.AssertEquals(1, alpha)

    red,green,blue,alpha = get_pixel(8,7)

    unit.AssertEquals(1, red)
    unit.AssertEquals(1, green)
    unit.AssertEquals(0, blue)
    unit.AssertEquals(1, alpha)

    unit.AssertThrows(image.SetPixel,image,-1,-2,colour.New(1,1,1))
    unit.AssertThrows(image.SetPixel,image,16,16,colour.New(1,1,1))
    unit.AssertThrows(image.SetPixel,image,2.5,1.25,colour.New(1,1,1))
end

module["Get pixel"] = function()
    set_pixel(0, 0, colour.New(1,0,1))
    set_pixel(15, 15, colour.New(0,1,1))
    set_pixel(8, 7, colour.New(1,1,0))

    local pixel = image:GetPixel(0,0)

    unit.AssertEquals(1, pixel.Red)
    unit.AssertEquals(0, pixel.Green)
    unit.AssertEquals(1, pixel.Blue)
    unit.AssertEquals(1, pixel.Alpha)

    pixel = image:GetPixel(15,15)

    unit.AssertEquals(0, pixel.Red)
    unit.AssertEquals(1, pixel.Green)
    unit.AssertEquals(1, pixel.Blue)
    unit.AssertEquals(1, pixel.Alpha)

    pixel = image:GetPixel(8,7)

    unit.AssertEquals(1, pixel.Red)
    unit.AssertEquals(1, pixel.Green)
    unit.AssertEquals(0, pixel.Blue)
    unit.AssertEquals(1, pixel.Alpha)

    unit.AssertThrows(image.GetPixel,image,-1,-2)
    unit.AssertThrows(image.GetPixel,image,16,16)
    unit.AssertThrows(image.GetPixel,image,2.5,1.25)
end

return module