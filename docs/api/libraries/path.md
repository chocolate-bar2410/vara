---
icon: octicons/rel-file-path-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-rel-file-path-24:</span> 
path
</h1>

Vara's library for creating and manipulating paths.

---

## functions

<h3 markdown>Join<span class="API-type"> -> string </span></h3>
```luau
path.Join = function(...: string): string
```
Concatenates the given strings into a file path.
<h3 markdown>WithExt<span class="API-type"> -> string </span></h3>
```luau
path.WithExt = function(
    path: string, 
    ext: string
): string
```
Adds a file extension onto a file path.
<h3 markdown>Basename<span class="API-type"> -> string </span></h3>
```luau
path.Basename = function(path: string): string
```
Gets the file name of a file path.
<h3 markdown>Absolute<span class="API-type"> -> string </span></h3>
```luau
path.Absolute = function(path: string): string
```
Converts a file path into an absolute file path.
<h3 markdown>Canonical<span class="API-type"> -> string </span></h3>
```luau
path.Canonical = function(path: string): string
```
Converts a file path into an canonical file path.
<h3 markdown>Glob<span class="API-type"> -> boolean </span></h3>
```luau
path.Glob = function(
    pattern: string,
    path: string
): boolean
```
Pattern matches a file path using a glob pattern.

| syntax | meaning |
|:------:|---------|
|?       |Matches any single character|
|*       |Matches zero or more characters, except for path seperators.|
|**      |Matches zero or more characters including path seperators|
|[ab]    |Matches one of the characters in the braces except path seperators, [a-z] for ranges, [!ab] or [^ab] to match any character but those ones.|
|{a,b}   |Matches one of the patterns in the braces.|
|!       |Negates the result.|
|\       |Escapes any of the other characters.|

see [fast-glob](https://crates.io/crates/fast-glob) for more info