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

function module.tables()
    
    unit.AssertTableEquals({1,2,3,4,5}, {1,2,3,4,5})
    unit.AssertTableEquals({
            hello = 123, 
                hi = {
                testing = 123
            }
        },{
            hello = 123, 
                hi = {
                testing = 123
            }
        }
    )

    local table_a = {1,2,3}
    local table_b = {1,2,3}
    local table_c = {test = table_a}
    local table_d = {test = table_b}

    table_a["test"] = table_c
    table_b["test"] = table_d

    unit.AssertTableEquals(table_c,table_d)


end

return module