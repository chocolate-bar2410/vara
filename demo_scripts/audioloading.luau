--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/Pink_noise.ogg")

fs.write("demo_scripts/audio/test.wav", audio:notch(500):fadeIn(2):fadeOut(2):encode("WAV"))