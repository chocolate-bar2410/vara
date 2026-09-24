--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/01 - blocksize 4096.flac")

print(audio)
fs.write("demo_scripts/audio/test.wav", audio:encode("WAV"))