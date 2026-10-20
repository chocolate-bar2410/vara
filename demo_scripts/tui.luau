--!strict
local chrono = require("@vara/chrono")
local fs = require("@vara/fs")
local io = require("@vara/io")
local system = require("@vara/system")
local task = require("@vara/task")
local terminal = require("@vara/terminal")

local style = terminal.Style
local colour = terminal.Colour

local screen_width,_screen_height = terminal.GetSize()
const SIZE_SUFFIXES = {"BYTES","KB","MB","GB","TB"}

local os = system.Os
    local language = system.LanguagePreferences.Message[1]

local format_byte_count = function(bytes: number)
    if bytes <= 0 then return "0.00 BYTES " end

    local index = math.min(
        math.floor(math.log(bytes,1024)),
        #SIZE_SUFFIXES - 1
    )
    local suffix = SIZE_SUFFIXES[index + 1];

    local truncated = bytes / (1024 ^ index);
    local size_string = string.format("%.2f %s",truncated,suffix)

    return size_string
end

local draw_box = function(x: number,y: number,width: number,height: number)
    terminal.MoveCursor(x,y)
    io.Write("╭")
    terminal.MoveCursor(x+width,y)
    io.Write("╮")
    terminal.MoveCursor(x,y+height)
    io.Write("╰")
    terminal.MoveCursor(x+width,y+height)
    io.Write("╯")

    terminal.MoveCursor(x + 1,y)
    io.Write(("─"):rep(width - 1))

    terminal.MoveCursor(x + 1,y+height)
    io.Write(("─"):rep(width - 1))

    io.Flush()

    if height <= 1 then return end

    for i = y + 1,y + height - 1 do 
        terminal.MoveCursor(x,i)
        io.Write("│")
        terminal.MoveCursor(x+width,i)
        io.Write("│")
    end

    io.Flush()
end

local render = function()
    terminal.Clear()
    terminal.MoveCursor(3,2)
    io.Write(`{colour.Magenta}vara TUI demo{style.Reset}`)
    terminal.MoveCursor(3,3)
    io.Write(`{colour.Magenta}{style.Dim}v{_RUNTIME.version.display}{style.Reset}`)

    fs.File("log.txt"):Append(buffer.fromstring(("%.f %.f\n"):format(screen_width / 2 - 2, 3)))

    draw_box(1, 1, screen_width - 1, 13)
    draw_box(2, 4, screen_width / 2 - 3, 3)

    draw_box(screen_width / 2 + 1, 4, screen_width / 2 - 2, 3)
    draw_box(2, 8, screen_width - 3, 5)

    terminal.MoveCursor(3,5)
    io.Write(`{colour.Magenta}Process memory{style.Reset}`)

    terminal.MoveCursor(screen_width / 2 + 2,5)
    io.Write(`{colour.Magenta}Logical CPUS{style.Reset}`)

    terminal.MoveCursor(3,9)
    io.Write(`{colour.Magenta}System Info{style.Reset}`)

    terminal.MoveCursor(3,9)
    io.Write(`{colour.Magenta}System Info{style.Reset}`)

    terminal.MoveCursor(3,11)
    io.Write(`{colour.Magenta}{style.Dim}OS/ARCH{style.Reset}`)
    terminal.MoveCursor(3,12)
    io.Write(`{colour.Magenta}{style.Dim}Language{style.Reset}`)

    terminal.MoveCursor(screen_width - 2 - #os,11)
    io.Write(`{colour.BrightCyan}{os}{style.Reset}`)
    terminal.MoveCursor(screen_width - 2 - #language,12)
    io.Write(`{colour.BrightCyan}{language}{style.Reset}`)

    io.Flush()
end

local thread = coroutine.create(function(...)
    while true do
        terminal.MoveCursor(3,6)
        io.Write(`{colour.BrightCyan}{format_byte_count(system.PhysicalMemoryUsage())}{style.Reset}`)
        terminal.MoveCursor(screen_width / 2 + 2,6)
        io.Write(`{colour.BrightCyan}{system.ProcessorCount()}{style.Reset}`)

        local date = chrono.Now():Humanise()

        terminal.MoveCursor(screen_width - #date ,2)
        io.Write(`{colour.BrightCyan}{date}{style.Reset}`)

        io.Flush()
        task.Sleep(1)
    end
end)

local resize_thread = coroutine.create(function(...)
    while true do
        local width,height = terminal.GetSize()

        if width ~= screen_width or height ~= _screen_height then
            screen_width = width
            _screen_height = height
            render()
        end

        task.Sleep(0.01)
    end
end)

terminal.EnterAltScreen()
terminal.EnterRawMode()
terminal.HideCursor()

render()

coroutine.resume(thread)
coroutine.resume(resize_thread)

repeat
    local key = io.ReadAsync(1) 
    
until key == string.char(3)

coroutine.close(resize_thread)
coroutine.close(thread)

terminal.ShowCursor()
terminal.ExitRawMode()
terminal.ExitAltScreen()