---
title: Input & Output
---

## Reading Inputs
The io library is designed to allow you to more control when reading and writing to the terminal.
Lets create our first interractive script:
```luau
const io = require("@vara/io")

print("what is your name")
local name = io.ReadLine()

print(`hello {name}`)
```

Save this file in your current directory, then run it with vara's CLI.
```
vara run hello.luau
```

## Writing Outputs
The above code may work, but maybe you want the user to write on the same line as the prompt.  
This is where `io.Write` comes in.  

If you run this script now, you'll notice that the message shows up after you sent a input.  
```luau
const io = require("@vara/io")

io.Write("what is your name:")
local name = io.ReadLine()

print(`hello {name}`)
```

This is because `io.Write` and `print` write to stdout. To output a message, stdout has to be flushed.  
Adding newlines with `\n` automatically flushes the output, but we dont want to newline in this case.

This is where we can use `io.Flush`.

```luau
const io = require("@vara/io")

io.Write("what is your name:")
io.Flush()
local name = io.ReadLine()

print(`hello {name}`)
```

Now if you run this script you should notice that you can write on the same line as the prompt.
