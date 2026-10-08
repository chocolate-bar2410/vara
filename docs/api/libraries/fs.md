---
icon: octicons/file-directory-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-file-directory-24:</span> 
fs
</h1>
Vara's library for managing the file system.

```luau
const fs = require("@vara/fs")

local source = buffer.fromstring("hello guys")
const path = "example.txt"

fs.Write(path,source)

local fileContents = fs.Read(path)
print(fileContents)
```

## functions

<h3 markdown>Write<span class="API-type"> -> () </span></h3>
```luau
fs.Write(
    path: string, 
    contents: buffer | string
): ()
```
Writes `contents` into the file that `path` points to.

<h3 markdown>Read<span class="API-type"> -> buffer </span></h3>
```luau
fs.Read(path: string): buffer
```
Reads the file contents of the file thats `path` points to.

<h3 markdown>File<span class="API-type"> -> File </span></h3>
```luau
fs.File(path: string): File
```
Creates a file object from a path string.

<h3 markdown>Directory<span class="API-type"> -> Directory </span></h3>
```luau
fs.Directory(path: string): Directory
```
Creates a directory object from a path string.
