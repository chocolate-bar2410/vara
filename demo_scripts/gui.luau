--!strict
local chrono = require("@vara/chrono")
local system = require("@vara/system")
local ui = require("@vara/ui")

local context = ui.CreateContext()

context:CreateWindow("Hi guys")
context:CreateWindow("こんにちは")

context:Run()