--!strict
local fs = require("@vara/fs")

local test_folder = fs.directory("demo_scripts/test")

test_folder:make():move("demo_scripts/moved")