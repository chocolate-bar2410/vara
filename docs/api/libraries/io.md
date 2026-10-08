---
icon: octicons/terminal-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-terminal-24:</span> 
io
</h1>

Vara's library for handling input/output.

```luau
const io = require("@vara/io")

io.Write("hi |")
io.Write("hello |")
io.Write("こんにちは |")
io.Write("write directly to stdout\n") --// newlines automatically flush stdout

const lineRead = io.ReadLine()
io.Write(`read entire line: {lineRead}\n`)

const readBytes = 256
const lineRead = io.Read(readBytes)
io.Write(`read 256 bytes: {lineRead}\n`)
```

---

## functions

<h3 markdown> Write <span class="API-type"> -> () </span></h3>
```luau
io.Write(message: string) : ()
```
Writes the string directly to stdout without flushing.

<h3 markdown> Read <span class="API-type"> -> string </span></h3>
```luau
io.Read(chunkSize: number?) : string
```
Reads from stdin, with a optional chunkSize value for reading a certain ammount of bytes.

<h3 markdown> ReadLine <span class="API-type"> -> string </span></h3>
```luau
io.ReadLine() : string
```
Reads stdin until it reaches a new line.