gui.registerFile("./demo_ui/test.slint")

local window = gui.createInstance("MainWindow")

window:connect("printmessage", function(message)
	print(message)
end)

window:run()
