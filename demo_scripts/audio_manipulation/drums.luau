--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audioA = media.Audio("demo_scripts/audio/drum_loop.flac")

fs.Write("demo_scripts/audio/drum_mastered.flac", 
    audioA
    :Saturate(2,"ARCTAN")
    :Compress({
        Input     = 4,
        Output    = 1,
        Threshold = -10,
        Ratio     = 2,
        Attack    = 0.14,
        Release   = 0.1,
        Knee      = 0
    })
    :Lowpass(1000)
    :Amplify(2)
    :Encode("FLAC")
)