--!strict
local fs = require("@vara/fs")
local image_file = fs.file("./demo_scripts/images/monarch.png")

local log_file = fs.file("./demo_scripts/log.txt")
local modified_time = log_file.modified

log_file:append(buffer.fromstring(`\z
    PATH: {image_file.path} \n\z
    MODIFIED AT: {modified_time:humanise()} \n\z
`))

log_file:copy("./demo_scripts/backup.txt")