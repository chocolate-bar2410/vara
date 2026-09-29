--!strict
local fs = require("@vara/fs")
local image_file = fs.file("./demo_scripts/images/monarch.png")

print(image_file.path)
print(image_file.size)

print(image_file:read())