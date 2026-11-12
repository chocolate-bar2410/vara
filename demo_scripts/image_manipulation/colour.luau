--!strict
local media = require("@vara/media")

local image = media.Image("demo_scripts/images/monarch.png")

const WIDTH = 64
const HEIGHT = 64

image
:Convolve(3, 3, {
    -1,0,1,
    -2,0,2,
    -1,0,1
})
:Resize(WIDTH, HEIGHT)
:Convolve(3, 3, {
    1/16,2/16,1/16,
    2/16,4/16,2/16,
    1/16,2/16,1/16,
})


for y = 0,WIDTH - 1 do
    for x = 0,HEIGHT - 1 do
        local pixel = image:GetPixel(x, y)
        pixel.Alpha = math.map(math.noise(x * 0.1,y * 0.1),-1,1,0,1)

        image:SetPixel(x, y,pixel)
    end 
end

local result = {}

local unicode = "　一二人口林画國繁龘驚鬱"

local function index_unicode(index: number): string
    local start = utf8.offset(unicode,index)

    local next_byte = utf8.offset(unicode,index + 1)
    return if next_byte ~= nil
        then unicode:sub(start,next_byte - 1)
        else unicode:sub(start,start)
end

for y = 0,WIDTH - 1 do
    for x = 0,HEIGHT - 1 do
        local pixel = image:GetPixel(x, y)

        local index = math.floor(pixel.Alpha * (utf8.len(unicode) :: number - 1)) + 1

        local character = index_unicode(index)

        result[#result + 1] = pixel:ToAnsi() .. character .. "\x1b[0m"
    end

    result[#result + 1] = "\n"
end

print(table.concat(result))