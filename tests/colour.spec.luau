--!strict
local colour = require("@vara/colour")
local unit = require("@vara/test/unit")

const EPSILON = 1e-6

local module = {}

function module.add()
    local a = colour.New(0.1, 0.2, 0.3, 1)
    local b = colour.New(0.3, 0.2, 0.1, 1)

    unit.AssertEquals(colour.New(0.4,0.4,0.4,1.0), a + b)
end

function module.subtract()
    local a = colour.New(0.1, 0.2, 0.3, 1)
    local b = colour.New(0.3, 0.2, 0.1, 1)

    local result = a - b
    local expected = colour.New(-0.2,0,0.2,0)

    unit.AssertTrue(math.abs(result.Red - expected.Red) < EPSILON)
    unit.AssertTrue(math.abs(result.Green - expected.Green) < EPSILON)
    unit.AssertTrue(math.abs(result.Blue - expected.Blue) < EPSILON)
    unit.AssertTrue(math.abs(result.Alpha - expected.Alpha) < EPSILON)
end

function module.multiply()
    local a = colour.New(0.1, 0.2, 0.3, 1)
    local b = colour.New(0.3, 0.2, 0.1, 1)

    local result = a * b
    local expected = colour.New(0.03,0.04,0.03,1)

    unit.AssertTrue(math.abs(result.Red - expected.Red) < EPSILON)
    unit.AssertTrue(math.abs(result.Green - expected.Green) < EPSILON)
    unit.AssertTrue(math.abs(result.Blue - expected.Blue) < EPSILON)
    unit.AssertTrue(math.abs(result.Alpha - expected.Alpha) < EPSILON)
end

function module.divide()
    local a = colour.New(0.1, 0.2, 0.3, 1)
    local b = colour.New(0.3, 0.2, 0.1, 1)

    local result = a / b
    local expected = colour.New(1/3,1,3,1)

    unit.AssertTrue(math.abs(result.Red - expected.Red) < EPSILON)
    unit.AssertTrue(math.abs(result.Green - expected.Green) < EPSILON)
    unit.AssertTrue(math.abs(result.Blue - expected.Blue) < EPSILON)
    unit.AssertTrue(math.abs(result.Alpha - expected.Alpha) < EPSILON)

    local c = colour.New(1,1,1,1)
    local d = colour.New(0,0,0,0)

    unit.AssertThrows(function()
        local _ = c / d
    end)
end

return module

