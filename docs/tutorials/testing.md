---
title: Writing tests
---

Testing is very important to make sure your code works as expected, and continues to work as you make changes.
Tests verify your functions, modules and applications behave correctly and handle edge cases approperiately.

## creating a test suite

Tests in vara are grouped by test suites, with each module file being its own test.
To create a test suite, simply create a module script with the `.spec.luau` file extension.

Every method of that module script will be recognised as a individual test.

```luau
--// test.spec.luau
const module = {}

function module["Add 2 numbers"]()

end

return module
```

## unit testing

The main way of testing most applications is unit testing. Unit testing break parts of an application or library into small isolated parts called units.
These parts are then tested individually for correctness.

Unit testing provides benefits such as:

* **safe refactoring**      - Tests ensure that changes to your code dont break functionality.
* **catching bugs early**   - Tests can find bugs that you might miss when writing code.
* **living documentation**  - Tests explicitly state how your code is supposed to behave.

for example lets test this block of code
```luau
function AddNumbers(A: number, B: number)
    return A + B
end
```

To do unit testing we can use the `@vara/test/unit` library.

```luau
--// test.spec.luau
const unit = require("@vara/test/unit")
const module = {}

--// code that we're going to test
local function AddNumbers(A: number, B: number)
    return A + B
end

function module["Add 2 numbers"]()
    unit.AssertEquals(2,AddNumbers(1,1))
    unit.AssertEquals(4,AddNumbers(2,2))

    unit.AssertThrows(AddNumbers,"not a number", "testing")
end

return module
```

To run our test we can then run:
```
vara run test.spec.luau
```

This ensures our `AddNumbers` method correctly adds together 2 numbers.
This example may seem simple but shows how you can apply these tests to your own application code.