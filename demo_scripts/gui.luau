local chrono = require("@vara/chrono")
local system = require("@vara/system")
local ui = require("@vara/ui")

local context = ui.CreateContext()

local inc_height = 0.05
local current_height = 0.03

local create_stat = function(contents: string)
    local text_label = context:TextLabel()

    text_label.Text = contents
    text_label.Size = {0.5,inc_height}
    text_label.Position = {0.05,current_height}

    text_label.BackgroundColour = {0,0,0,0}
    text_label.TextColour = {1,1,1,0.5}

    text_label.TextSize = 20

    current_height += inc_height + 0.005
end

--// UI

local frame = context:Frame()

frame.Size = {0.95,0.95}
frame.Position = {0.025,0.025}
frame.Colour = {0.05,0.05,0.05,0}

frame.CornerRadius = 0.025
frame.BorderWidth = 0.0025
frame.BorderColour = {1,1,1,0.5}

create_stat("Welcome to vara")
create_stat("------------------------------------------------------------------")
create_stat(`Date : {chrono.Now():Humanise()}`)
create_stat(`OS : {system.Os} {system.Distro} {system.Arch}`)
create_stat(`DE : {system.DesktopEnvironment}`)
create_stat(`Language : {system.LanguagePreferences.Message[1]}`)


context:Run()