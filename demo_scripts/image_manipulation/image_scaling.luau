--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local input_file = fs.File("demo_scripts/images/cameraman.png")

local bilinear = fs.File("demo_scripts/images/resizing_demo/cameraman_bilinear.png")
local nearest = fs.File("demo_scripts/images/resizing_demo/cameraman_nearest.png")
local lanczos3 = fs.File("demo_scripts/images/resizing_demo/cameraman_lanczos3.png")

bilinear:Write(
    media.Image(input_file)
        :Resize(512,512,"BILINEAR")
        :Encode("PNG")
)

nearest:Write(
    media.Image(input_file)
        :Resize(512,512,"NEAREST")
        :Encode("PNG")
)

lanczos3:Write(
    media.Image(input_file)
        :Resize(512,512,"LANCZOS3")
        :Encode("PNG")
)