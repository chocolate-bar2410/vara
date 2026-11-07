---
title: test
---

# Usage
```pwsh
vara test <path_to_directory>
vara test <path_to_script.spec.luau>
```

Evaluates the specified .spec.luau files.  
If a directory is given, it will recursively evaluate every .spec.luau file in that directory.

## how test suites works

Every .spec.luau file is a suite, which acts as a module script.
Every method of that module is a test, which will be evaluated.

=== "testing.spec.luau"
    ```luau
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
    ```
=== "output"
    ```
    SUITE testing
    PASS testing > addition
    PASS testing > subtraction

    2 pass
    0 fail
    ```