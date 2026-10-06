---
title: io
---

Vara's library for handling input/output.

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