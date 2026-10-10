--!strict
local fs = require("@vara/fs")
local system = require("@vara/system")

const BLUE = "\x1b[34m"
const GREY = "\x1b[90m"

const BOLD = "\x1b[1m"
const DIM = "\x1b[2m"
const RESET = "\x1b[0m"

--// use a nerd font like JetBrainsMono Nerd Font
const FOLDER_ICON = "\u{f114}"
const IMAGE_ICON = "\u{f03e}"
const FILE_ICON = "\u{f4a5}"

local image_extensions = {
    png = true,
    jpeg = true,
    jpg = true,
    qoi = true,
    bmp = true,
    ppm = true,
}


local function printFolderTree(folder: Directory,prefix: string?)
    if prefix == nil then
        local relative = folder.Path:match("[^\\]+$") or folder.Path
        print(`{FOLDER_ICON}  {BLUE}{BOLD}{relative}{RESET}`)
    end

    prefix = prefix or ""

    local file_entries = {}

    for file in folder:Walk() do
        table.insert(file_entries,file) 
    end

    for index,file : File | Directory in file_entries do
        
        local is_last = index == #file_entries

        local branch = is_last and "└──" or "├──"
        local child_prefix = is_last and "    " or "│   "

        local relative = file.Path:match("[^\\]+$") or file.Path
        if typeof(file) == "File" then

            local icon = image_extensions[file.Extension] and IMAGE_ICON or FILE_ICON

            print(`{GREY}{DIM}{prefix}{branch}{RESET} {icon}  {relative}`)
        else
            print(`{GREY}{DIM}{prefix}{branch}{RESET} {FOLDER_ICON}  {BLUE}{BOLD}{relative}{RESET}`)
            printFolderTree(file,child_prefix)
        end
        
    end
end

local args = system.Args

if #args < 1 then
    error("no arguments provided")
end

local test_folder = fs.Directory(args[1])

if test_folder.Exists == false then
    error(`{test_folder.Path} does not exist`)
end

printFolderTree(test_folder)