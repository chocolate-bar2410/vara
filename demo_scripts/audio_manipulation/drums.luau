--!strict
local fs = require("@vara/fs")
local media = require("@vara/media")

local audioA = media.audio("demo_scripts/audio/drum_loop.flac")

fs.Write("demo_scripts/audio/drum_mastered.flac", 
    audioA
    :saturate(2,"ARCTAN")
    :compress({
        input     = 4,
        output    = 1,
        threshold = -10,
        ratio     = 2,
        attack    = 0.14,
        release   = 0.1,
        knee      = 0
    })
    :lowpass(1000)
    :amplify(2)
    :encode("FLAC")
)