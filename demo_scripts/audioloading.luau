--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/sample-3s-stereo.wav")

fs.write("demo_scripts/audio/test.wav", audio:normalise(-5):fadeIn(3):fadeOut(1):encode("WAV"))