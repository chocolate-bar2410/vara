-- for testing the testing framework
local unit = require("@vara/test/unit")

local module = {}

function module.addition()
    
    unit.AssertEquals(2, 1 + 1)
    
end

function module.subtraction()
    print("subtraction")
end

return module