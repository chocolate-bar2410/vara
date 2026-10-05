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
    media.audio({
        tracks = {data,data},
        sampleRate = SAMPLE_RATE,
        frames = DATA_SIZE // 4
    })
    :fadeIn(0.01)
    :fadeOut(0.3)
    :compress({
        output = 1,
        threshold = -12,
        attack = 0.02,
        release = 0.12,
        ratio = 3,
        knee = 2
    })
    :saturate(1,"TANH")
    :softClip(-3)
    :lowpass(1000)
    :encode("FLAC")
)