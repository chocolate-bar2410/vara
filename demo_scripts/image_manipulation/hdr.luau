--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local output_file = fs.File("demo_scripts/images/hdr/hdr_image.png")

local image = media.Image("demo_scripts/images/hdr/qwantani_dusk_2_puresky_1k.hdr")

for y = 0, image.Height - 1 do
    for x = 0, image.Width - 1 do
        image:SetPixel(x, y,image:GetPixel(x, y):HdrTonemap())
    end 
end

output_file:Write(image:Encode("PNG"))

print(`successfully written to {output_file.Path}`)

