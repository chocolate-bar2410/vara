local chrono = require("@vara/chrono")

local timestamp = chrono.now()
timestamp.timeOffset = -5.5

print(timestamp)

print(`ISO8601:   {timestamp:toISO8601()}`)
print(`RFC2822:   {timestamp:toRFC2822()}`)
print(`humanised: {timestamp:humanise()}`)
print(`中国:      {timestamp:toZH()}`)
print(`Apache:   {timestamp:toApacheLog()}`)