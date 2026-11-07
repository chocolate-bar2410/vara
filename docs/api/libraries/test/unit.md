---
icon: lucide/brick-wall
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:lucide-brick-wall:</span> 
unit
</h1>

Vara's unit testing library.  
For assertions and other unit testing functionality.
example code
```luau
local unit = require("@vara/test/unit")
local module = {}

function AddNumbers(A: number, B: number)
    return A + B
end

function module["Add 2 numbers together"]()
    unit.AssertEquals(2,AddNumbers(1,1))
    unit.AssertEquals(4,AddNumbers(2,2))

    unit.AssertThrows(AddNumbers,"hi guys", "omg")
end


return module
```


---

## functions

<h3 markdown>AssertEquals<span class="API-type"> -> () </span></h3>
```luau
unit.AssertEquals<T>(
    expected: T,
    actual: T
)
```
Asserts if a value is equal to a expected value.

<h3 markdown>AssertNotEquals<span class="API-type"> -> () </span></h3>
```luau
unit.AssertNotEquals<T>(
    expected: T,
    actual: T
)
```
Asserts if a value is not equal to a expected value.

<h3 markdown>AssertTrue<span class="API-type"> -> () </span></h3>
```luau
unit.AssertTrue(condition: boolean)
```
Asserts if a condition is true.

<h3 markdown>AssertNotNil<span class="API-type"> -> () </span></h3>
```luau
unit.AssertNotNil<T>(value: T)
```
Asserts if a value is not nil.

<h3 markdown>AssertThrows<span class="API-type"> -> () </span></h3>
```luau
unit.AssertThrows<T>(
    callback: () -> (),
    ... : T
)
```
Asserts if a function throws an error.

<h3 markdown>AssertThrows<span class="API-type"> -> () </span></h3>
```luau
unit.AssertDoesNotThrow<T>(
    callback: () -> (),
    ... : T
)
```
Asserts if a function does not throw an error.

<h3 markdown>AssertTableEquals<span class="API-type"> -> () </span></h3>
```luau
unit.AssertTableEquals<K,V>(
    expected: {[K]: V},
    actual: {[K]: V}
)
```
Asserts if a table is equal to a expected table.

<h3 markdown>AssertTableNotEquals<span class="API-type"> -> () </span></h3>
```luau
unit.AssertTableNotEquals<K,V>(
    expected: {[K]: V},
    actual: {[K]: V}
)
```
Asserts if a table is not equal to a expected table.

