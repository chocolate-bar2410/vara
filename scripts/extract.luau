--[[
extraction script

for when you need to compact the typedef folder into a single .txt file.

]]
local fs = require("@vara/fs")
local system = require("@vara/system")

local typedef_dir = fs.Directory("typedef")

local extracted = {}

function extract(directory: Directory)
    for file in directory:Walk() do
        
        if typeof(file) == "Directory" then 
            extract(file) 
        else
            table.insert(extracted,buffer.tostring(file:Read()))
        end
    end
end

extract(typedef_dir)

local source = table.concat(extracted,"\n")
local clipboard = system.Clipboard()

clipboard:SetText(source)

print("\x1b[1;32mFiles successfully extracted\x1b[0m \x1b[36mFile contents added to clipboard\x1b[0m")