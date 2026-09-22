local fs = require("@vara/fs")
local media = require("@vara/media")

local image = media.image("demo_scripts/images/monarch.png")

local sobel_x_3x3 = {
    -1, 0, 1,
    -10, 0, 10,
    -1, 0, 1
}

local sobel_y_3x3 = {
   -1,-10,-1,
    0, 0, 0,
    1, 10, 1,
}


fs.write(
    "demo_scripts/images/monarch_convolve3x3.png", 
    image
    :greyscale()
    :convolve(3,3,sobel_x_3x3)
    :convolve(3,3,sobel_y_3x3)
    :encode("PNG")
)