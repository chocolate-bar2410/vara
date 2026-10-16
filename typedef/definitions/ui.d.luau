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
end
