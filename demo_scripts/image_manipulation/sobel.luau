--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local sobel_x_3x3 = {
    -1, 0, 1,
    -2, 0, 2,
    -1, 0, 1
}

local sobel_y_3x3 = {
   -1,-2,-1,
    0, 0, 0,
    1, 2, 1,
}

local gaussian = {
    1/16,2/16,1/16,
    2/16,4/16,2/16,
    1/16,2/16,1/16
}

local image = media.Image("demo_scripts/images/monarch.png"):Greyscale():Convolve(3, 3, gaussian)

local image_x = image:Clone():Convolve(3,3,sobel_x_3x3):Pow(2)
local image_y = image:Clone():Convolve(3,3,sobel_y_3x3):Pow(2)

local result = image_x:Add(image_y):Pow(0.5)

result.Channels = 1

fs.Write(
    "demo_scripts/images/monarch_edge.png", 
    result:Encode("PNG")
)
