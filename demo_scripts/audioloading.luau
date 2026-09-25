--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/01 - blocksize 4096.flac")

fs.write("demo_scripts/audio/test.wav", audio:highpass(1000):fadeIn(3):fadeOut(1):encode("WAV"))