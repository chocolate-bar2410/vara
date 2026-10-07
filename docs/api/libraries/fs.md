---
title: fs
---

Vara's library for accessing the file system.

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
