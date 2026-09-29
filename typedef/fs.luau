--[=[
	@class fs

	Built in library for managing the file system.

	### example code
	```luau
	const fs = require("@vara/fs")

	local source = buffer.fromstring("hello guys")
	const path = "example.txt"

	fs.write(path,source)

	local fileContents = fs.read(path)
	print(fileContents)
	```
]=]
const module = {}

--[=[
	@within fs
	@method write
	@param path The file being written to.
	@param contents The contents of the file.

	Writes `contents` into the file that `path` points to.
]=]
function module.write(path: string, contents: buffer | string): ()
	return nil :: unknown
end

--[=[
	@within fs
	@method write
	@param path The file being written to.
	@return The contents of the file.

	Reads the file contents of the file thats `path` points to.
]=]
function module.read(path: string): buffer
	return nil :: unknown
end

--[=[
	@within fs
	@method file
	@param path The path to a file.
	@return A file object.

	Creates a file object from a path string.
]=]
function module.file(path: string): File
	return nil :: unknown
end

return module
