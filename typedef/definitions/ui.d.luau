--[=[
	@class UiContext

    Object that handles user interfaces.
]=]
declare extern type UiContext with
    --[=[
        @within UiContext
        @method Run
        
        Starts rendering the UI.
    ]=]
    function Run(self) : ()

    --[=[
        @within UiContext
        @method Frame
        
        Creates a frame UI component.
    ]=]
    function Frame(self) : UiFrame
end

--[=[
	@class UiFrame

    A UI component representing a plain frame.
]=]
declare extern type UiFrame with
    Visible: boolean
    Position: {number}
    Size: {number}
    Colour: {number}
end
