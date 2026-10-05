--!strict
local chrono = require("@vara/chrono")

local current = os.time()
local timestampA = chrono.FromUnix(current)
local timestampB = chrono.FromUnix(current)

timestampA.TimeOffset = 10

print(`A = B:     {timestampA == timestampB}`)
print(`A < B:     {timestampA < timestampB}`)
print(`A ≤ B:     {timestampA <= timestampB}`)

local timestampC = chrono.Now()

print(`ISO8601:   {timestampC:ToISO8601()}`)
print(`RFC2822:   {timestampC:ToRFC2822()}`)
print(`humanised: {timestampC:Humanise()}`)
print(`中国:      {timestampC:ToZH()}`)
print(`Apache:   {timestampC:ToApacheLog()}`)

local timestampD = chrono.FromUniversal({
    Year = 2025,
    Month = 9,
    Day = 10,

    Hour = 18,
    Minute = 20,
    Second = 0,
    Millisecond = 0,
})

timestampD.TimeOffset = 10
print(timestampD:Humanise())