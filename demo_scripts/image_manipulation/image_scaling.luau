--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local input_file = fs.File("demo_scripts/images/cameraman.png")

local bilinear = fs.File("demo_scripts/images/resizing_demo/cameraman_bilinear.png")
local nearest = fs.File("demo_scripts/images/resizing_demo/cameraman_nearest.png")
local lanczos3 = fs.File("demo_scripts/images/resizing_demo/cameraman_lanczos3.png")

bilinear:Write(
    media.image(input_file)
        :resize(512,512,"BILINEAR")
        :encode("PNG")
)

nearest:Write(
    media.image(input_file)
        :resize(512,512,"NEAREST")
        :encode("PNG")
)

lanczos3:Write(
    media.image(input_file)
        :resize(512,512,"LANCZOS3")
        :encode("PNG")
)