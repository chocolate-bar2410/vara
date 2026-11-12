---
title: Quickstart
---

## overview
Build a minimal CLI application with vara.
This will assume your using visual studio code with the [luau lsp](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.luau-lsp) extension.

---

**step 1**  
Initialise a new project with `vara setup`.
```terminal
vara setup
```

**step 2**  
Create a new script `main.luau`

```luau linenums="1"
const system = require("@vara/system")
const terminal = require("@vara/terminal")
const args = system.Args

local command = args[1]

if command ~= "echo" then
    warn("invalid command")
    return
end

print(terminal.Colour.Cyan .. (args[2] or "???") .. terminal.Style.Reset)
```

**step 3**  
run the file with `vara run main.luau`

=== "terminal"
    ```terminal
    vara run main.luau -- echo "hello world"
    ```
=== "output"
    ```
    hello world
    ```

