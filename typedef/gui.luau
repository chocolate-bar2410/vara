--[=[
	@within gui
	@interface UIInstance

	A reference to a slint gui instance.

	* `get/set` - To access and modify properties.
	* `getGlobal/setGlobal` - To access and modify globals.
	* `connect` - To connect a function to a callback.
	* `connectGlobal` - To connect a function to a global callback.
	* `run` - To start rendering the UI.

]=]
export type UIInstance = {
	run: (self: UIInstance) -> (),
	get: <T>(self: UIInstance, property: string) -> T,
	set: <T>(self: UIInstance, property: string, value: T) -> (),
	connect: (self: UIInstance, callbackName: string, (...unknown) -> unknown) -> (),

	getGlobal: <T>(self: UIInstance, global: string, property: string) -> T,
	setGlobal: <T>(self: UIInstance, global: string, property: string, value: T) -> (),
	connectGlobal: (self: UIInstance, global: string, callbackName: string, (...unknown) -> unknown) -> (),
}

--[=[
	@class gui

	Built in library for managing graphical user interfaces and [slint](https://slint.dev/) components.

	### example code
	```luau
	const gui = require("@vara/gui")

	gui.registerFile("components.slint") 

	const window = gui.createInstance("MainWindow")

	window:connect("onButtonPress", function()
		print("Button was pressed!")
	end)

	window:connect("checkIsEven", function(num : number)
		return num % 2 == 0
	end)

	window:run()
	```
]=]
const module = {}

--[=[
	@within gui
	@method registerFile
	@param path Path to .slint file.

	Registers slint component definitions into vara.
]=]
function module.registerFile(path: string): ()
	return nil :: unknown
end

--[=[
	@within gui
	@method registerFile
	@param component Component definition
	@return UIInstance

	Creates a new UI instance from a component definition.

	### example code
	```luau
	const gui = require("@vara/gui")

	gui.registerFile("components.slint") 

	const window = gui.createInstance("MainWindow")
	window:run()
	```
]=]
function module.createInstance(component: string): UIInstance
	return nil :: unknown
end

return module
