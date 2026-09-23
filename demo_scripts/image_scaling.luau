local fs = require("@vara/fs")
local media = require("@vara/media")

fs.write(
    "demo_scripts/images/cameraman_scaled.png", 
    media.image("demo_scripts/images/cameraman.png")
        :resize(2048,2048)
        :encode("PNG")
)