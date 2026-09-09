gui.registerFile("./demo_ui/test.slint")

local window = gui.createInstance("MainWindow")

window:set("message","こんにちは")

window:run()