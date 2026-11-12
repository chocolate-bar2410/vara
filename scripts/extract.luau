--[[
extraction script

for when you need to compact the typedef folder into a single .txt file.

]]
local fs = require("@vara/fs")

local extract_file = fs.File("extract.txt")
local typedef_dir = fs.Directory("typedef")

function extract(directory: Directory)
    for file in directory:Walk() do
        
        if typeof(file) == "Directory" then 
            extract(file) 
        else
            extract_file:Append(file:Read())
        end
    end
end

extract(typedef_dir)
print("\x1b[1;32mFiles successfully extracted\x1b[0m")