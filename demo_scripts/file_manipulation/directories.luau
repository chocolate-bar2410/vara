--!strict
local fs = require("@vara/fs")
local io = require("@vara/io")

const BLUE = "\x1b[34m"
const BOLD = "\x1b[1m"
const RESET = "\x1b[0m"

--// use a nerd font like JetBrainsMono Nerd Font
const FOLDER_ICON = "\u{f114}"
const IMAGE_ICON = "\u{f03e}"

io.write("\x1b[2J\x1b[H\n")

local function printFolderTree(folder: Directory,prefix: string?)
    if prefix == nil then
        local relative = folder.path:match("[^\\]+$") or folder.path
        print(`{FOLDER_ICON}  {BLUE}{BOLD}{relative}{RESET}`)
    end

    prefix = prefix or ""

    local file_entries = {}

    for file in folder:walk() do
        table.insert(file_entries,file) 
    end

    for index,file : File | Directory in file_entries do
        
        local is_last = index == #file_entries

        local branch = is_last and "└──" or "├──"
        local child_prefix = is_last and "    " or "│   "

        local relative = file.path:match("[^\\]+$") or file.path

        if typeof(file) == "File" then
            print(`{prefix}{branch} {IMAGE_ICON}  {relative}`)
        else
            print(`{prefix}{branch} {FOLDER_ICON}  {BLUE}{BOLD}{relative}{RESET}`)
            printFolderTree(file,child_prefix)
        end
        
    end
end

local test_folder = fs.directory("demo_scripts/images")

printFolderTree(test_folder)
io.readline()