---
icon: octicons/file-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-file-24:</span> 
File
</h1>
An object representing a file

## properties

<h3 markdown>Path<span class="API-type"> : string</span></h3>

The absolute path to the file.

<h3 markdown>Size<span class="API-type"> : number</span></h3>

The size of the file in bytes.

<h3 markdown>Created<span class="API-type"> : DateTime</span></h3>

The time the file was created.

<h3 markdown>Accessed<span class="API-type"> : DateTime</span></h3>

The time the file was accessed.

<h3 markdown>Modified<span class="API-type"> : DateTime</span></h3>

The time the file was modified.

<h3 markdown>Exists<span class="API-type"> : boolean</span></h3>

Whether or not the file exists yet.

<h3 markdown>Extension<span class="API-type"> : boolean</span></h3>

The file extension of the file.

## methods

<h3 markdown>Read<span class="API-type"> -> number </span></h3>
```luau
File:Read(): buffer
```
Reads the file as a buffer of unsigned 8 bit integers.

<h3 markdown>Write<span class="API-type"> -> number </span></h3>
```luau
File:Write(contents : buffer) : File
```
Writes the contents of the file. 
Returns itself.

<h3 markdown>Append<span class="API-type"> -> number </span></h3>
```luau
File:Append(contents : buffer) : File
```
Appends the given contents onto the end of the contents of the file.
Returns itself.

<h3 markdown>Delete<span class="API-type"> -> number </span></h3>
```luau
File:Delete()
```
Deletes the file.

<h3 markdown>Move<span class="API-type"> -> number </span></h3>
```luau
File:Move(path: string) : File
```
Moves the file to a different location.
Returns itself.

<h3 markdown>Copy<span class="API-type"> -> number </span></h3>
```luau
File:Copy(path: string) : File
```
Copies the file to a new location.
Returns the copied file.

## metamethods

<h3 markdown>__tostring<span class="API-type"> -> string </span></h3>

Converts the file into a string in the format `file [Size] [Path]`
