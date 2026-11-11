---
title: File system
---
When making applications, you are eventually going to need to access files.  
Vara's file system library is designed to make managing files intuitive for applications.

## Files
Unlike other runtimes, [`files`](../api/objects/file.md) are objects that can be owned and used by our scripts.

```luau
const fs = require("@vara/fs")

local my_file = fs.File("message.txt")
```

Reading and writing files can be done using the `Read` and `Write` methods.  
Note that in vara, files work with buffers not with strings.  
This may be unorthodox but is really useful once you start working with binary files such as parquet or msgpack files.

```luau
const fs = require("@vara/fs")

local my_file = fs.File("message.txt")

my_file:Write(buffer.fromstring("こんにちは！!"))

local source = buffer.tostring(my_file:Read())
print(source)
```

We can also append data onto files.

```luau
const fs = require("@vara/fs")

local my_file = fs.File("log.txt")

my_file:Append(buffer.fromstring("Hello guys\n"))
my_file:Append(buffer.fromstring("こんにちは\n"))
my_file:Append(buffer.fromstring("你好\n"))
```
This becomes very useful when we cannot output debug messages to the terminal.

## directories
Like with files, [`directories`](../api/objects/directory.md) are also objects.

```luau
const fs = require("@vara/fs")

local my_dir = fs.Directory("example_folder")

-- use directory:Make() to make that folder exist in your file system.
my_dir:Make()
```

We can walk through a directory to get every file/directory inside it.
!!! note
    Currently walking is none recursive, to recursively walk through a directory you will have to handle every directory you encounter.

```luau
const fs = require("@vara/fs")

local my_dir = fs.Directory("example_folder")

for file: File | Directory in my_dir:Walk() do
    print(`found {typeof(file)} {file.Path}`)
end
```

## file observers
Eventually we will want to use our programs to automate file processing.
We may want to immediately move a file or process a file if its in a certain directory.
Instead of polling our directory every few seconds, we can instead bind functions to certain file events and run those functions.

We can do this by using the [`FsObserver`](../api/objects/fsobserver.md) object.

```luau
const fs = require("@vara/fs")

local my_dir = fs.Directory("example_folder")
local observer = fs.Observer(my_dir)
```

### file automation
Now lets make a program that automatically sorts files in our downloads folder.

Lets first set up our folders.
```luau
--// init.luau
const fs = require("@vara/fs")
const system = require("@vara/system") -- module for getting system info
const path = require("@vara/path") -- module for manipulating paths

local downloads = fs.Directory(path.Join(system.Home,"Downloads"))

local photo_dir = fs.Directory(path.Join(downloads.Path,"photos"))
local music_dir = fs.Directory(path.Join(downloads.Path,"music"))
local misc_dir = fs.Directory(path.Join(downloads.Path,"misc"))
```

Lets also create a module script that we can use to assign different file extensions to different folders.
```luau
--// extensions.luau
const module = {}

module.photos = {
    png = true
    jpeg = true
    bmp = true
}

module.music = {
    wav = true
    flac = true
    ogg = true
}

return module
```

we can now create a function that sorts our files

```luau
--// init.luau
local extensions = require("extensions")

local function sort_files()
    for file in downloads:Walk() do
        if file.Path == music_dir.Path then continue end
        if file.Path == photo_dir.Path then continue end
        if file.Path == misc_dir.Path then continue end

        if typeof(file) == "Directory" then
            file:Move(path.Join(misc_dir.Path,path.BaseName(file.Path)))
            continue
        end

        if extensions.music[file.Extension] then
            file:Move(path.Join(music_dir.Path,path.BaseName(file.Path)))
            continue
        end

        if extensions.photos[file.Extension] then
            file:Move(path.Join(photo_dir.Path,path.BaseName(file.Path)))
            continue
        end

        file:Move(path.Join(misc_dir.Path,path.BaseName(file.Path)))
    end
end
```

We can then bind that function to a file event.
```luau
sort_files() -- to sort already existing files

local observer = fs.Observer(downloads)
observer.OnCreated = sort_files

repeat
until io.ReadAsync(1) -- io.Read variant that doesn't block the task scheduler.

-- observers must be closed after your done using them
observer:Disconnect()
```
