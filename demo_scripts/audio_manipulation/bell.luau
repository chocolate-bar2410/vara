--!strict
local media = require("@vara/media")
local fs = require("@vara/fs")

const SAMPLE_RATE: number = 44100;
const DURATION: number = 2;

const FREQUENCY: number = 261.63 * 2;
const AMPLITUDE: number = 1;

const DATA_SIZE = SAMPLE_RATE * DURATION * 4

local carrier = 0
local modulator = 0
local sample = 0

local carrier_offset = math.tau * FREQUENCY / SAMPLE_RATE
local mod_offset = math.tau * FREQUENCY * 2.7 / SAMPLE_RATE

local bellOscillator = function()
    local t = sample / SAMPLE_RATE
    local amplitude = AMPLITUDE * math.exp(-2.5 * t)
    local modulation_depth = 8 * math.exp(-2 * t)

    local value = amplitude * math.sin(
        carrier  
        + math.sin(modulator) * modulation_depth 
        + math.sin(modulator * 2) * 0.5 *  modulation_depth
    )

    carrier += carrier_offset
    modulator += mod_offset
    sample += 1

    return value
end

local data = buffer.create(DATA_SIZE)

for i = 0,DATA_SIZE - 1,4 do
    buffer.writef32(data, i, bellOscillator())
end

fs.Write("demo_scripts/audio/bell.flac",
    media.Audio({
        Tracks = {data,data},
        SampleRate = SAMPLE_RATE,
        Frames = DATA_SIZE // 4
    })
    :FadeIn(0.01)
    :FadeOut(0.3)
    :Compress({
        Output = 1,
        Threshold = -12,
        Attack = 0.02,
        Release = 0.12,
        Ratio = 3,
        Knee = 2
    })
    :Saturate(1,"TANH")
    :SoftClip(-3)
    :Lowpass(1000)
    :Encode("FLAC")
)