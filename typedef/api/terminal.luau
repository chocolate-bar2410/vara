--[=[
    @class terminal

    Library for terminal manipulation.
]=]
local module = {}

--[=[
    @within terminal
    @prop Style

    Look up table for ansi style codes.
]=]
module.Style = {
    ["Reset"]          = "\x1b[0m",
    ["Bold"]           = "\x1b[1m",
    ["Dim"]            = "\x1b[2m",
    ["Italic"]         = "\x1b[3m",
    ["Underline"]      = "\x1b[4m",
    ["Blink"]          = "\x1b[5m",
    ["Invert"]         = "\x1b[7m",
    ["Hidden"]         = "\x1b[8m",
    ["StrikeThrough"]  = "\x1b[9m",
}

--[=[
    @within terminal
    @prop Colour
    
    Look up table for ansi colour codes.
]=]
module.Colour = {
    ["Red"]     = "\x1b[31m",
    ["Green"]   = "\x1b[32m",
    ["Yellow"]  = "\x1b[33m",
    ["Blue"]    = "\x1b[34m",
    ["Magenta"] = "\x1b[35m",
    ["Cyan"]    = "\x1b[36m",
    ["White"]   = "\x1b[37m",

    ["BackgroundBlack"]   = "\x1b[40m",
    ["BackgroundRed"]     = "\x1b[41m",
    ["BackgroundGreen"]   = "\x1b[42m",
    ["BackgroundYellow"]  = "\x1b[43m",
    ["BackgroundBlue"]    = "\x1b[44m",
    ["BackgroundMagenta"] = "\x1b[45m",
    ["BackgroundCyan"]    = "\x1b[46m",
    ["BackgroundWhite"]   = "\x1b[47m",

    ["BrightBlack"]   = "\x1b[90m",
    ["BrightRed"]     = "\x1b[91m",
    ["BrightGreen"]   = "\x1b[92m",
    ["BrightYellow"]  = "\x1b[93m",
    ["BrightBlue"]    = "\x1b[94m",
    ["BrightMagenta"] = "\x1b[95m",
    ["BrightCyan"]    = "\x1b[96m",
    ["BrightWhite"]   = "\x1b[97m",

    ["BackgroundBrightBlack"]   = "\x1b[100m",
    ["BackgroundBrightRed"]     = "\x1b[101m",
    ["BackgroundBrightGreen"]   = "\x1b[102m",
    ["BackgroundBrightYellow"]  = "\x1b[103m",
    ["BackgroundBrightBlue"]    = "\x1b[104m",
    ["BackgroundBrightMagenta"] = "\x1b[105m",
    ["BackgroundBrightCyan"]    = "\x1b[106m",
    ["BackgroundBrightWhite"]   = "\x1b[107m",
}

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