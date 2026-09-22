local media = require("@vara/media")
local fs = require("@vara/fs")

local data = fs.read("./demo_scripts/images/test.ppm")

-- print(buffer.tostring(data))

local image = media.decodeImage("PNM",data)
print(image)