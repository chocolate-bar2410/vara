local fs = require("@vara/fs")
local media = require("@vara/media")

local image = media.image("./demo_scripts/images/lenna.png")

fs.write("./demo_scripts/images/lenna_flip.png", image:flip(0):encode("PNG"))