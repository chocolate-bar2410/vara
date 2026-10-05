--[=[
	@class io
	Built in library for handling stdio.

	### example usage

	```luau
	const io = require("@vara/io")

	io.Write("hi |")
	io.Write("hello |")
	io.Write("こんにちは |")
	io.Write("write directly to stdout\n") --// newlines automatically flush stdout

	const lineRead = io.ReadLine()
	io.Write(`read entire line: {lineRead}\n`)

	const readBytes = 256
	const lineRead = io.Read(readBytes)
	io.Write(`read 256 bytes: {lineRead}\n`)
	```
]=]
const module = {}
--[=[
	@within io
	@method Write
	@param message The message being sent to stdout.
	
	Writes a message to stdout buffer.
]=]
function module.Write(message: string): ()
	return nil :: unknown
end

--[=[
	@within io
	@method Read
	@param chunkSize The ammount of bytes being read, reads 1024 bytes if no size is given.
	@tag must_use
	@return string
	
	Reads `chunkSize` bytes from the stdout buffer.
]=]
function module.Read(chunkSize: number?): string
	return nil :: unknown
end

--[=[
	@within io
	@method ReadLine
	@tag must_use
	@return string
	
	Reads an entire line from the stdout buffer.
]=]
function module.ReadLine(): string
	return nil :: unknown
end

return module
