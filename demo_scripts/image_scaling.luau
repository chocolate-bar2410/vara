local fs = require("@vara/fs")
local media = require("@vara/media")

fs.write(
    "demo_scripts/images/cameraman_bilinear.png", 
    media.image("demo_scripts/images/cameraman.png")
        :resize(2048,2048,"BILINEAR")
        :encode("PNG")
)

fs.write(
    "demo_scripts/images/cameraman_nearest.png", 
    media.image("demo_scripts/images/cameraman.png")
        :resize(2048,2048,"NEAREST")
        :encode("PNG")
)