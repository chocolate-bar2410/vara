--[=[
    @class terminal

    Library for terminal manipulation.
]=]
local module = {}


--[=[
    @within terminal
    @method Clear

    Clears the terminal
]=]
module.Clear = function()
    return nil :: unknown
end

--[=[
    @within terminal
    @method HideCursor

    Hides the cursor.
]=]
module.HideCursor = function()
    return nil :: unknown
end

--[=[
    @within terminal
    @method HideCursor

    Shows the cursor.
]=]
module.ShowCursor = function()
    return nil :: unknown
end

--[=[
    @within terminal
    @method MoveCursor
    @param x The column to move to cursor to. 
    @param y The line to move the cursor to.
    Shows the cursor.
]=]
module.MoveCursor = function(x: number,y: number)
    return nil :: unknown
end

return module