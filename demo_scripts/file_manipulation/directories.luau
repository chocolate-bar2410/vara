--!strict
local fs = require("@vara/fs")

local test_folder = fs.directory("demo_scripts/test")

print(test_folder:make())