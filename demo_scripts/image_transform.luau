local fs = require("@vara/fs")
local media = require("@vara/media")

local image = media.image("demo_scripts/images/frymire.png")

fs.write("demo_scripts/images/frymire_grey.png", image:greyscale():encode("PNG"))