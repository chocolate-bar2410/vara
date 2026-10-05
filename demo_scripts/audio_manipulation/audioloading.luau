--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audioA = media.audio("demo_scripts/audio/Pink_noise.ogg")
local audioB = media.audio("demo_scripts/audio/sample-3s-16khz.wav")

fs.Write("demo_scripts/audio/test_16khz.wav", 
    audioA:saturate(12,"ARCTAN")
          :notch(500)
          :trim(0,5)
          :lowpass(3000)
          :fadeIn(2)
          :fadeOut(2)
          :encode("WAV")
)