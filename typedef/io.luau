--[=[
	@class io
	Built in library for handling stdio.

	### example usage

	```luau
	const io = require("@vara/io")

	io.write("hi |")
	io.write("hello |")
	io.write("こんにちは |")
	io.write("write directly to stdout\n") --// newlines automatically flush stdout

	const lineRead = io.readline()
	io.write(`read entire line: {lineRead}\n`)

	const readBytes = 256
	const lineRead = io.read(readBytes)
	io.write(`read 256 bytes: {lineRead}\n`)
	```
]=]
const module = {}
--[=[
	@within io
	@method write
	@param message The message being sent to stdout.
	
	Writes a message to stdout buffer.
]=]
function module.write(message: string): ()
	return nil :: unknown
end

--[=[
	@within io
	@method read
	@param chunkSize The ammount of bytes being read, reads 1024 bytes if no size is given.
	@tag must_use
	@return string
	
	Reads `chunkSize` bytes from the stdout buffer.
]=]
function module.read(chunkSize: number?): string
	return nil :: unknown
end

--[=[
	@within io
	@method readline
	@tag must_use
	@return string
	
	Reads an entire line from the stdout buffer.
]=]
function module.readline(): string
	return nil :: unknown
end

return module
