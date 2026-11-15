--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local image = media.Image("demo_scripts/images/monarch.png")

local image_a = image:Clone():Convolve(3, 3, {
    47/512,0,-47/512,
    162/512,0,-162/512,
    47/512,0,-47/512,
}):Pow(2)

local image_b = image:Clone():Convolve(3, 3, {
    47/512,162/512,47/512,
    0,0,0,
    -47/512,-162/512,-47/512,
}):Pow(2)

image_a:Add(image_b):Pow(0.5)
-- image_a.Channels = 1

fs.File("demo_scripts/images/diffgaussian.png"):Write(image_a:Encode("PNG"))
