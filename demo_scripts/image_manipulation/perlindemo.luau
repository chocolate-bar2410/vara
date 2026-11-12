--!strict
const media = require("@vara/media")
const fs = require("@vara/fs")

local height = 256
local width = 256
local channels = 4
local bytedepth = 4

local data = buffer.create(height * width * channels * bytedepth)
const scale = 0.15

local function perlin_noise(x: number,y: number,octaves: number,lacunarity: number,persistence: number)
    local result : number = 0
    local max_amplitude = 0
    local amplitude = 1
    local frequency = scale

    for i = 1,octaves do
        local value = math.noise(x * frequency,y * frequency,12345)

        result += value * amplitude
        max_amplitude += amplitude
        amplitude *= persistence
        frequency *= lacunarity
    end

    result /= max_amplitude
    result = math.map(result,-1,1,0,1)
    return result
end

local offset = 0

for y = 0,height - 1 do
    for x = 0,width - 1 do

        local value = perlin_noise(x, y, 4, 0.5, 2)

        -- print(value,x,y)

        buffer.writef32(data,offset,value)
        buffer.writef32(data,offset + 1 * bytedepth,value)
        buffer.writef32(data,offset + 2 * bytedepth,value)
        buffer.writef32(data,offset + 3 * bytedepth,1)

        offset += 4 * bytedepth
    end
end

fs.Write("./demo_scripts/images/basn2c16.qoi",
    media.Image(data)
        :Dimensions(width,height)
        :Encode("QOI")
)