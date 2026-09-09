gui.registerFile("./demo_ui/test.slint")

local message = gui.createInstance("MainWindow"):get("message")

print(message)