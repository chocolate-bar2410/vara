local gui = require("@vara/gui")

gui.registerFile("./demo_ui/test.slint")

local window = gui.createInstance("MainWindow")

window:connectGlobal("Testing", "onClick", function(message)
	if message == "" then
		return
	end
	print(message)
end)

window:run()
