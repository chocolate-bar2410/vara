local fs = require("@vara/fs")
local media = require("@vara/media")

local image = media.image("demo_scripts/images/Lenna.png")

fs.write("demo_scripts/images/Lenna_flipped.qoi", image:flip():encode("QOI"))