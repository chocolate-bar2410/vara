local fs = require("@vara/fs")
local media = require("@vara/media")

local input_file = fs.file("demo_scripts/images/cameraman.png")

local bilinear = fs.file("demo_scripts/images/resizing_demo/cameraman_bilinear.png")
local nearest = fs.file("demo_scripts/images/resizing_demo/cameraman_nearest.png")
local lanczos3 = fs.file("demo_scripts/images/resizing_demo/cameraman_lanczos3.png")

bilinear:write(
    media.image(input_file)
        :resize(512,512,"BILINEAR")
        :encode("PNG")
)

nearest:write(
    media.image(input_file)
        :resize(512,512,"NEAREST")
        :encode("PNG")
)

lanczos3:write(
    media.image(input_file)
        :resize(512,512,"LANCZOS3")
        :encode("PNG")
)