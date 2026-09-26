--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audioA = media.audio("demo_scripts/audio/Pink_noise.ogg")
local audioB = media.audio("demo_scripts/audio/sample-3s-16khz.wav")

fs.write("demo_scripts/audio/test_16khz.wav", 
    audioA:resample(audioB.sampleRate,"LANCZOS3")
          :trim(0,5)
          :overlay(audioB,0,true)
          :fadeIn(2)
          :fadeOut(2)
          :encode("WAV")
)