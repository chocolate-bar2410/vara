local chrono = require("@vara/chrono")

local current = os.time()
local timestampA = chrono.fromUnix(current)
local timestampB = chrono.fromUnix(current)

timestampA.timeOffset = 10

print(`A = B:     {timestampA == timestampB}`)
print(`A < B:     {timestampA < timestampB}`)
print(`A ≤ B:     {timestampA <= timestampB}`)

local timestampC = chrono.now()

print(`ISO8601:   {timestampC:toISO8601()}`)
print(`RFC2822:   {timestampC:toRFC2822()}`)
print(`humanised: {timestampC:humanise()}`)
print(`中国:      {timestampC:toZH()}`)
print(`Apache:   {timestampC:toApacheLog()}`)