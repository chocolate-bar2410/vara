---
title: Directory
---

An object representing a file directory.

## properties

<h3 markdown>Path<span class="API-type"> : string</span></h3>

The absolute path to the directory.

<h3 markdown>Created<span class="API-type"> : DateTime</span></h3>

The time the directory was created.

<h3 markdown>Accessed<span class="API-type"> : DateTime</span></h3>

The time the directory was accessed.

<h3 markdown>Modified<span class="API-type"> : DateTime</span></h3>

The time the directory was modified.

<h3 markdown>Exists<span class="API-type"> : boolean</span></h3>

Whether or not the directory exists yet.

## methods

<h3 markdown>Make<span class="API-type"> -> File </span></h3>
```luau
Directory:Make() : File
```

Creates a file directory where the directory object's path is located.

<h3 markdown>Move<span class="API-type"> -> File </span></h3>
```luau
Directory:Move(path: string) : File
```

Moves the directory to a different location.

<h3 markdown>Delete<span class="API-type"> -> () </span></h3>
```luau
Directory:Delete() : ()
```

Deletes the file directory and all files inside it.

<h3 markdown>Walk<span class="API-type"> -> () -> File | Directory </span></h3>
```luau
Directory:Walk() : () -> File | Directory
```

Returns an non recursive iterator that walks through every file and directory in that directory.

## metamethods

<h3 markdown>__tostring<span class="API-type"> -> string </span></h3>

Converts the file into a string in the format `directory [Path]`