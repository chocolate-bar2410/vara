--[=[
	@class serde
	Built in library for handling serialising and deserialising data.

	### example usage
	```luau
	local serde = require("@vara/serde")

	local message = serde.encode("JSON", { hello = "world", testing = 123 })
	print(message)

	local data : { hello: string, testing: number } = serde.decode("JSON",message)
	print(data.hello)
	print(data.testing)
	```
]=]
const module = {}

type SerdeFormat = "JSON"
--[=[
	@within serde
	@method encode
	@param data The data being encoded.
	@param format support formats: `JSON`.
	@tag must_use
	@return string
	
	Encodes the data provided into a specified file format.
]=]
function module.encode(format: SerdeFormat, data: unknown): string
	return nil :: unknown
end

--[=[
	@within serde
	@method decode
	@param message The message being sent to stdout.
	@param format support formats: `JSON`.
	@tag must_use
	@return unknown
	
	Decode the data provided from a specified file format into luau values.
]=]
function module.decode(format: SerdeFormat, data: string): unknown
	return nil :: unknown
end

return module
