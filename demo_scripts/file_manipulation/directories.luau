--!strict
local fs = require("@vara/fs")
local io = require("@vara/io")

const BLUE = "\x1b[34m"
const GREY = "\x1b[90m"

const BOLD = "\x1b[1m"
const DIM = "\x1b[2m"
const RESET = "\x1b[0m"

--// use a nerd font like JetBrainsMono Nerd Font
const FOLDER_ICON = "\u{f114}"
const IMAGE_ICON = "\u{f03e}"

io.Write("\x1b[2J\n")

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
            print(`{GREY}{DIM}{prefix}{branch}{RESET} {IMAGE_ICON}  {relative}`)
        else
            print(`{GREY}{DIM}{prefix}{branch}{RESET} {FOLDER_ICON}  {BLUE}{BOLD}{relative}{RESET}`)
            printFolderTree(file,child_prefix)
        end
        
    end
end

local test_folder = fs.Directory("demo_scripts/images")

printFolderTree(test_folder)
io.ReadLine()
io.Write("\x1b[2J\n")