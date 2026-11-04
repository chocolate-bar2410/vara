local unit = require("@vara/test/unit")

local module = {}

function module.addition()
    error("addition failed")
    -- print("addition")
end

function module.subtraction()
    print("subtraction")
end

return module