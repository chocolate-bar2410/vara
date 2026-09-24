--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/sample-3s-16khz.wav")

fs.write("demo_scripts/audio/test.wav", audio:amplify(3):encode("WAV"))