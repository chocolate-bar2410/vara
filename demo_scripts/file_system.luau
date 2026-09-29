--!strict
local chrono = require("@vara/chrono")
local fs = require("@vara/fs")
local image_file = fs.file("./demo_scripts/images/monarch.png")

local log_file = fs.file("./demo_scripts/log.txt")
local access_time = log_file.accessed

log_file:append(buffer.fromstring(`\z
    PATH: {image_file.path} \n\z
    ACCESSED AT: {access_time:humanise()} \n\z
`))