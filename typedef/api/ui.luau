--[=[
	@class ui

	WARNING: THIS LIBRARY IS WINDOWS ONLY, THIS API DOES NOT HAVE SUPPORT FOR LINUX OR MACOS YET.

	Inbuilt library for creating native user interfaces.
]=]
const module = {}
--[=[
	@within ui
	@method CreateContext
	
	Creates a new ui application
]=]
function module.CreateContext(): UiApplication
	return nil :: unknown
end

return module
