--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/sample-3s-stereo.wav")

print(audio)
fs.write("demo_scripts/audio/test.flac", audio:encode("FLAC"))