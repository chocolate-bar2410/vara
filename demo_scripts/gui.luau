--!strict
local chrono = require("@vara/chrono")
local system = require("@vara/system")
local ui = require("@vara/ui")

local context = ui.CreateContext()

local window_A = context:CreateWindow("Hi guys")
local window_B = context:CreateWindow("こんにちは")

context:Run()