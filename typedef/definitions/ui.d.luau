--[=[
	@class UiContext

    Object that handles user interfaces.
]=]
declare extern type UiContext with
    Resizable: boolean
    Width: number
    Height: number
    BackgroundColour: {number}
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

    --[=[
        @within UiContext
        @method TextLabel
        
        Creates a text label UI component.
    ]=]
    function TextLabel(self) : UiTextLabel
end

--[=[
	@class UiFrame

    A UI component representing a container.
]=]
declare extern type UiFrame with
    Visible: boolean
    Position: {number}
    Size: {number}
    Colour: {number}
    CornerRadius: number
    BorderWidth: number
    BorderColour: {number}
end

--[=[
	@class UiTextLabel

    Displays text on the UI.
]=]
declare extern type UiTextLabel with
    Text: string
    TextSize: number

    Visible: boolean
    Position: {number}
    Size: {number}
    TextColour: {number}
    BackgroundColour: {number}

    CornerRadius: number
    BorderWidth: number
    BorderColour: {number}
end