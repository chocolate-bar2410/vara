const media = require("@vara/media")
const fs = require("@vara/fs")

local data: buffer = fs.read("./demo_scripts/images/basn2c16.png")
local image: Image = media.decodeImage("PNM",data)

fs.write("./demo_scripts/images/basn2c16.qoi",image:encode("QOI"))