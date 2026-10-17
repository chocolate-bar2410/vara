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
    @method WithExt
    @param path The file path.
    @return The name of the file.

    Gets the file name of a file path.
]=]
module.Basename = function(path: string): string
    return nil :: unknown
end

return module