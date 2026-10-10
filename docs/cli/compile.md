---
title: compile
---

# Usage
```pwsh
vara compile <path_to_script.luau>
```

Compiles your script into a `.exe` or equivalent file format used by your operating system.

!!! info
    Currently compilation does not support the `require` function for requiring other luau files.  
    if you want to compile multiple scripts you must use a bundler like larvae or darklua to bundle your code.  

    This feature is currently planned and will come out at a future version.