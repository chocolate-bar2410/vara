--[[
api generation script

When you need to create a new api definition file.

]]
local fs = require("@vara/fs")
local path = require("@vara/path")
local system = require("@vara/system")
local terminal = require("@vara/terminal")
local args = system.Args

local style = terminal.Style

if args[1] == nil then
    error("no arguments were provided")
end

local template = [[--[=[
    @class __MODULE__

    generated library.
]=]
local module = {}

--[=[
    @within __MODULE__
    @method example

    generated function.
]=]
module.example = function()
    return nil :: unknown
end

return module
]]

local library_name = args[1]
local source = buffer.fromstring(template:gsub("__MODULE__", library_name))

local lib_path = path.Join("typedef","api",library_name)
lib_path = path.WithExt(lib_path,"luau")

fs.File(lib_path):Write(source)

print(`API definition for library {style.Italic}{style.Bold}'{library_name}'{style.Reset} has been created.`)