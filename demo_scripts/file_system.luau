--!strict
local chrono = require("@vara/chrono")
local fs = require("@vara/fs")
local image_file = fs.file("./demo_scripts/images/monarch.png")

local access_time = chrono.now()
local log_file = fs.file("./demo_scripts/log.txt")

log_file:append(buffer.fromstring(`\z
    PATH: {image_file.path} \n\z
    ACCESSED AT: {access_time:humanise()} \n\z
`))