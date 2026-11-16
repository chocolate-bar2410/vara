--[=[
	@class UiApplication

    Application object that handles user interfaces.
]=]
declare extern type UiApplication with
    --[=[
        @within UiApplication
        @method CreateWindow
        @param title The title of the window.
        @return The window create.

        Creates a new window
    ]=]
    function CreateWindow(self,title: string): UiWindow
    --[=[
        @within UiApplication
        @method Run

        Starts rendering the user interface.
    ]=]
    function Run(self)
end

--[=[
	@class UiWindow

    Window object that holds user interface elements inside it.
]=]
declare extern type UiWindow with
    Destroyed: boolean
    OnClose: () -> boolean?
    --[=[
        @within UiWindow
        @emthod Close

        Closes the window.
    ]=]
    function Close(self)
end