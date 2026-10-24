--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local output_file = fs.File("demo_scripts/images/hdr/hdr_image.png")

local image = media.Image("demo_scripts/images/hdr/qwantani_dusk_2_puresky_1k.hdr")

image:HdrTonemap({
    Filter = "ACES",
    Exposure = -3
})

output_file:Write(image:Encode("PNG"))

print(`successfully written to {output_file.Path}`)

