--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local image = media.Image("demo_scripts/images/monarch.png")

image:Convolve(5, 5, {
    1/273,4/273,7/273,4/273,1/273,
    4/273,16/273,26/273,16/273,4/273,
    7/273,26/273,41/273,26/273,7/273,
    4/273,16/273,26/273,16/273,4/273,
    1/273,4/273,7/273,4/273,1/273,
})

local image_b = image:Clone():Convolve(5, 5, {
    1/273,4/273,7/273,4/273,1/273,
    4/273,16/273,26/273,16/273,4/273,
    7/273,26/273,41/273,26/273,7/273,
    4/273,16/273,26/273,16/273,4/273,
    1/273,4/273,7/273,4/273,1/273,
})

image:Subtract(image_b)

image.Channels = 1
fs.File("demo_scripts/images/diffgaussian.png"):Write(image:Encode("PNG"))
