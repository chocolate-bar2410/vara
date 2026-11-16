--[=[
	@class UiApplication

    Application object that handles user interfaces.
]=]
declare extern type UiApplication with
    function CreateWindow(self,title: string): UiWindow
    function Run(self)
end

--[=[
	@class UiWindow

    Window object that holds user interface elements inside it.
]=]
declare extern type UiWindow with
    Destroyed: boolean
    function close(self)
end