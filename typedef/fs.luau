--[=[
	@class fs

	Built in library for managing the file system.

	### example code
	```luau
	const fs = require("@vara/fs")

	local source = buffer.fromstring("hello guys")
	const path = "example.txt"

	fs.Write(path,source)

	local fileContents = fs.Read(path)
	print(fileContents)
	```
]=]
const module = {}

--[=[
	@within fs
	@method Write
	@param path The file being written to.
	@param contents The contents of the file.

	Writes `contents` into the file that `path` points to.
]=]
function module.Write(path: string, contents: buffer | string): ()
	return nil :: unknown
end

--[=[
	@within fs
	@method Read
	@param path The file being written to.
	@return The contents of the file.

	Reads the file contents of the file thats `path` points to.
]=]
function module.Read(path: string): buffer
	return nil :: unknown
end

--[=[
	@within fs
	@method File
	@param path The path to a file.
	@return A file object.

	Creates a file object from a path string.
]=]
function module.File(path: string): File
	return nil :: unknown
end

--[=[
	@within fs
	@method Directory
	@param path The path to a file directory.
	@return A directory object.

	Creates a directory object from a path string.
]=]
function module.Directory(path: string): Directory
	return nil :: unknown
end

return module
