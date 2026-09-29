--!strict
local fs = require("@vara/fs")

local image_folder = fs.directory("demo_scripts/images")

print(image_folder)