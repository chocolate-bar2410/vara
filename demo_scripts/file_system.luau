--!strict
local fs = require("@vara/fs")
local image = fs.file("./demo_scripts/images/monarch.png")

print(image.path)
print(image.size)