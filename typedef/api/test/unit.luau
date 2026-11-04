--[=[
    @class unit

    For assertions and other unit testing functionality.
]=]
const module = {}

--[=[
    @within unit
    @method AssertEquals
    @param expected The value to be expected.
    @param actual The value thats being compared.

    Asserts if a value is equal to a expected value.
]=]
function module.AssertEquals<T>(expected: T,actual: T)
    return nil :: unknown
end


return module