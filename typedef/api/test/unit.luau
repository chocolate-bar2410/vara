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

--[=[
    @within unit
    @method AssertNotEquals
    @param expected The value to be expected.
    @param actual The value thats being compared.

    Asserts if a value is not equal to a expected value.
]=]
function module.AssertNotEquals<T>(expected: T,actual: T)
    return nil :: unknown
end


--[=[
    @within unit
    @method AssertTrue
    @param condition The condition being tested.

    Asserts if a condition is true.
]=]
function module.AssertTrue(condition: boolean)
    return nil :: unknown
end


--[=[
    @within unit
    @method AssertNotNil
    @param value The value to be tested.

    Asserts if a value is not nil.
]=]
function module.AssertNotNil<T>(value: T)
    return nil :: unknown
end

--[=[
    @within unit
    @method AssertThrows
    @param callback The function to be tested.
    @param ... arguments to be passed in.

    Asserts if a function throws an error.
]=]
function module.AssertThrows<T>(callback: () -> (),... : T)
    return nil :: unknown
end

--[=[
    @within unit
    @method AssertThrows
    @param callback The function to be tested.
    @param ... arguments to be passed in.

    Asserts if a function does not throw an error.
]=]
function module.AssertDoesNotThrow<T>(callback: () -> (),... : T)
    return nil :: unknown
end

--[=[
    @within unit
    @method AssertTableEquals
    @param expected The table to be expected.
    @param actual The table thats being compared.

    Asserts if a table is equal to a expected table.
]=]
function module.AssertTableEquals<K,V>(expected: {[K]: V},actual: {[K]: V})
    return nil :: unknown
end

--[=[
    @within unit
    @method AssertTableNotEquals
    @param expected The table to be expected.
    @param actual The table thats being compared.

    Asserts if a table is not equal to a expected table.
]=]
function module.AssertTableNotEquals<K,V>(expected: {[K]: V},actual: {[K]: V})
    return nil :: unknown
end


return module