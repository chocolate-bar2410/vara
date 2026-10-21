--[[
extraction script

for when you need to compact the typedef folder into a single .txt file.

]]
local fs = require("@vara/fs")

local extract_file = fs.File("extract.txt")

local api_dir = fs.Directory("typedef/api")
local definition_dir = fs.Directory("typedef/definitions")


for file in api_dir:Walk() do
    if typeof(file) == "Directory" then continue end

    extract_file:Append(file:Read())
end


for file in definition_dir:Walk() do
    if typeof(file) == "Directory" then continue end

    extract_file:Append(file:Read())
end
