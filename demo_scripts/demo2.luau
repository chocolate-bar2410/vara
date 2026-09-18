local chrono = require("@vara/chrono")

local timestamp = chrono.now()

print(timestamp:toISO8601())