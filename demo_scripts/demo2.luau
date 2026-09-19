local chrono = require("@vara/chrono")

local current = os.time()
local timestampA = chrono.fromUnix(current)
local timestampB = chrono.fromUnix(current)

timestampA.timeOffset = 10

print(timestampA == timestampB)
print(timestampA < timestampB)
print(timestampA <= timestampB)


print(timestampA)

-- print(`ISO8601:   {timestampA:toISO8601()}`)
-- print(`RFC2822:   {timestampA:toRFC2822()}`)
-- print(`humanised: {timestampA:humanise()}`)
-- print(`中国:      {timestampA:toZH()}`)
-- print(`Apache:   {timestampA:toApacheLog()}`)


