--!strict
local fs = require("@vara/fs")
local image_file = fs.File("./demo_scripts/images/monarch.png")

local log_file = fs.File("./demo_scripts/log.txt")
local modified_time = log_file.Modified

log_file:Append(buffer.fromstring(`\z
    PATH: {image_file.Path} \n\z
    MODIFIED AT: {modified_time:humanise()} \n\z
`))

log_file:Copy("./demo_scripts/backup.txt")