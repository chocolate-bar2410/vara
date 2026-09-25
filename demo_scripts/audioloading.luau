--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audio = media.audio("demo_scripts/audio/01 - blocksize 4096.flac")
print(audio.sampleRate)
fs.write("demo_scripts/audio/test_16khz.wav", audio:resample(16000,"LINEAR"):fadeIn(2):fadeOut(2):encode("WAV"))