--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audioA = media.Audio("demo_scripts/audio/Pink_noise.ogg")
local audioB = media.Audio("demo_scripts/audio/sample-3s-16khz.wav")

fs.Write("demo_scripts/audio/test_16khz.wav", 
    audioA:Saturate(12,"ARCTAN")
          :Notch(500)
          :Trim(0,5)
          :Lowpass(3000)
          :FadeIn(2)
          :FadeOut(2)
          :Encode("WAV")
)