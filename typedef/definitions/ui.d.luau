--[=[
	@class UiApplication

    Application object that handles user interfaces.
]=]
declare extern type UiApplication with
    --[=[
        @within UiApplication
        @method CreateWindow
        @param title The title of the window.
        @return The window created.

        Creates a new window.
    ]=]
    function CreateWindow(self,title: string,min_size: vector?,max_size: vector?): UiWindow
    --[=[
        @within UiApplication
        @method CreateButton
        @param window Which window is the button being assigned to.
        @param text Text displayed by the button.
        @return The Button created.

        Creates a new button.
    ]=]
    function CreateButton(self,window: UiWindow,text: string,position: vector?,size: vector?): ()
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
    OnResize: (width: number,height: number) -> ()
    --[=[
        @within UiWindow
        @emthod Close

        Closes the window.
    ]=]
    function Close(self)
end