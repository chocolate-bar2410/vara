local fs = require("@vara/fs")
local media = require("@vara/media")

fs.write(
    "demo_scripts/images/resizing_demo/cameraman_bilinear.png", 
    media.image("demo_scripts/images/cameraman.png")
        :resize(512,512,"BILINEAR")
        :encode("PNG")
)

fs.write(
    "demo_scripts/images/resizing_demo/cameraman_nearest.png", 
    media.image("demo_scripts/images/cameraman.png")
        :resize(512,512,"NEAREST")
        :encode("PNG")
)

fs.write(
    "demo_scripts/images/resizing_demo/cameraman_lanczos3.png", 
    media.image("demo_scripts/images/cameraman.png")
        :resize(512,512,"LANCZOS3")
        :encode("PNG")
)