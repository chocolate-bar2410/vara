--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local output_file = fs.File("demo_scripts/images/hdr/hdr_image.png")

local image = media.Image("demo_scripts/images/hdr/ferndale_studio_12_1k.hdr")

image:HdrTonemap({
    Filter = "UCHIMURA",
    Exposure = 0.2,
    Contrast = 0.9,

})

output_file:Write(image:Encode("PNG"))

print(`successfully written to {output_file.Path}`)

