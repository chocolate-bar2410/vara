local chrono = require("@vara/chrono")

local timestamp = chrono.now()

print(timestamp:toISO8601())
print(timestamp:toRFC2822())
print(timestamp:humanise())
print(timestamp:toZH())
print(timestamp:toApacheLog())