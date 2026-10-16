local ui = require("@vara/ui")

local context = ui.CreateContext()

local frame = context:Frame()

frame.Size = {0.9,0.9}
frame.Position = {0.5 - frame.Size[1]/4,0.5 - frame.Size[2]/4}
frame.Colour = {0.05,0.05,0.05,0}

frame.CornerRadius = 0.025
frame.BorderWidth = 0.0025
frame.BorderColour = {0.1,0.1,0.1,1}

context:Run()