const media = require("@vara/media")
const fs = require("@vara/fs")

local image: Image = media.image("./demo_scripts/images/basn2c16.png")

fs.write("./demo_scripts/images/basn2c16.qoi",image:encode("QOI"))