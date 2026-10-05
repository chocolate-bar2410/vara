--[=[
	@class serde
	Built in library for serialising and deserialising data.

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

type SerdeFormat = "JSON" | "TOML" | "YAML"
--[=[
	@within serde
	@method Encode
	@param data The data being encoded.
	@param format Support formats: `JSON`, `TOML` & `YAML`.
	@tag must_use
	@return string
	
	Encodes the data provided into a specified file format.
]=]
function module.Encode(format: SerdeFormat, data: unknown): string
	return nil :: unknown
end

--[=[
	@within serde
	@method Decode
	@param message The message being sent to stdout.
	@param format Support formats: `JSON`, `TOML` & `YAML`.
	@tag must_use
	@return unknown
	
	Decode the data provided from a specified file format into luau values.
]=]
function module.Decode(format: SerdeFormat, data: string): unknown
	return nil :: unknown
end

return module
