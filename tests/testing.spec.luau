-- for testing the testing framework
local unit = require("@vara/test/unit")

local module = {}

function module.addition()
    
    unit.AssertEquals(2, 1 + 1)
    unit.AssertNotEquals(3, 2 + 2)
    
    unit.AssertThrows(function()
        error("testing 123")
    end)

end

function module.subtraction()
    unit.AssertEquals(1, 2 - 1)
    unit.AssertNotEquals(3, 2 - 2)
end

return module