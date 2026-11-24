--!strict
local system = require("@vara/system")
local media = require("@vara/media")

local clipboard = system.Clipboard()

local image = media.Image("demo_scripts/system/clipboard.png")

clipboard:SetImage(image)