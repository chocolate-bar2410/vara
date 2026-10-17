--[=[
    @class path

    Library for managing file paths
]=]
local module = {}


--[=[
    @within path
    @method Join
    @param ... The strings being concatenated to create a file path.
    @return A file path

    Concatenates the given strings into a file path.
]=]
module.Join = function(...: string): string
    return nil :: unknown
end

--[=[
    @within path
    @method WithExt
    @param path The file path.
    @param ext The file extension being added.
    @return A file path with an extension

    Adds a file extension onto a file path.
]=]
module.WithExt = function(path: string, ext: string): string
    return nil :: unknown
end

--[=[
    @within path
    @method Basename
    @param path The file path.
    @return The name of the file.

    Gets the file name of a file path.
]=]
module.Basename = function(path: string): string
    return nil :: unknown
end

--[=[
    @within path
    @method Absolute
    @param path The file path.
    @return The name of the file.

    Converts a file path into an absolute file path.
]=]
module.Absolute = function(path: string): string
    return nil :: unknown
end

--[=[
    @within path
    @method Glob
    @param pattern The pattern being used
    @param path The file path being matched
    @return Whether is successfully matches

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

]=]
module.Glob = function(pattern: string, path: string): boolean
    return nil :: unknown
end


return module