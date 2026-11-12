--!strict
local io = require("@vara/io")
local task = require("@vara/task")
local terminal = require("@vara/terminal")

local style = terminal.Style
local colour = terminal.Colour

local exit_text = "[CTRL+C] Quit"
local generate_text = "[SPACE] Generate"

const MIN_WIDTH = 30
const MIN_HEIGHT = 26

const module = {}

module.screen_width, module.screen_height = terminal.GetSize()

module.current_pallete = {} :: {Colour4}
module.pallete_string = ""

module.resize_thread = (nil :: unknown) :: thread

local get_length_ignore_ansi = function(str: string): number
    return #str:gsub("\x1b%[[0-?]*[ -/]*[@-~]","")
end

module.init = function()
    terminal.EnterAltScreen()
    terminal.EnterRawMode()
    terminal.HideCursor()

    module.resize_thread = coroutine.create(function()
        while true do
            module.on_resize()
            task.Sleep(0.05)
        end
    end)

    coroutine.resume(module.resize_thread)
end

module.exit = function()
    coroutine.close(module.resize_thread)

    terminal.ShowCursor()
    terminal.ExitRawMode()
    terminal.ExitAltScreen()
end

module.alt_render = function()
    terminal.Clear()

    local middle_x = module.screen_width // 2
    local middle_y = module.screen_height // 2 - 2

    local message = "Terminal size too small:"
    local message_x = math.max(1,middle_x - get_length_ignore_ansi(message) // 2)

    local height_colour = if module.screen_height < MIN_HEIGHT then colour.Red else colour.Green
    local width_colour = if module.screen_width < MIN_WIDTH then colour.Red else colour.Green

    local indicator = `width = {width_colour}{module.screen_width}{style.Reset} height = {height_colour}{module.screen_height}{style.Reset}`
    local indicator_x = math.max(1,middle_x - get_length_ignore_ansi(indicator) // 2)

    local requirement_message = `Needed dimensions are:`
    local requirement_message_x = math.max(1,middle_x - get_length_ignore_ansi(requirement_message) // 2)
    
    local requirement = `width = {MIN_WIDTH} height = {MIN_HEIGHT}`
    local requirement_x = math.max(1,middle_x - get_length_ignore_ansi(requirement) // 2)
    
    terminal.MoveCursor(message_x,middle_y)
    io.Write(message)
    terminal.MoveCursor(indicator_x,middle_y + 1)
    io.Write(indicator)
    terminal.MoveCursor(requirement_message_x,middle_y + 4)
    io.Write(requirement_message)
    terminal.MoveCursor(requirement_x,middle_y + 5)
    io.Write(requirement)

    io.Flush()
end

module.render = function()
    
    if module.screen_width < MIN_WIDTH or module.screen_height < MIN_HEIGHT then
        module.alt_render()
        return
    end

    io.Write("\x1b[?2026h")
    io.Flush()
    
    terminal.Clear()
    
    terminal.MoveCursor(1, 1)
    io.Write(`{style.Bold}Chroma{style.Reset}`)  

    terminal.MoveCursor(1, 2)
    io.Write(("-"):rep(module.screen_width)) 

    terminal.MoveCursor(1, module.screen_height - 1)
    io.Write(generate_text)

    terminal.MoveCursor(module.screen_width - #exit_text, module.screen_height - 1)
    io.Write(exit_text)

    local halfwidth = module.screen_height // 2

    module.draw_box(1,halfwidth,module.screen_width - 1,halfwidth - 2)

    if #module.current_pallete > 0 then
        local pallete = module.current_pallete
        local main_colour = pallete[1]
        local main_ansi = main_colour:ToAnsi()

        local red, green, blue = main_colour:ToSRGBA()

        red = math.max(math.floor(main_colour.Red * 255),0.0)
        green = math.max(math.floor(main_colour.Green * 255),0.0)
        blue = math.max(math.floor(main_colour.Blue * 255),0.0)

        terminal.MoveCursor(3, halfwidth + 1)
        io.Write("Selected colour")

        module.draw_filled_rect(3,halfwidth + 3,6,3,main_ansi)

        terminal.MoveCursor(10, halfwidth + 3)
        io.Write(`#{style.Bold}{pallete[1]:ToHex()}{style.Reset}`)
        terminal.MoveCursor(10, halfwidth + 4)
        io.Write((`RGB {style.Bold}%d %d %d{style.Reset}`):format(red,green,blue))
        terminal.MoveCursor(10, halfwidth + 5)
        io.Write((`OKLCH {style.Bold}%.2f %.2f %d°{style.Reset}`):format(main_colour:ToOKLCH()))

        
        module.render_pallete_grid()

    else
        terminal.MoveCursor(3, halfwidth + 1)
        io.Write("Selected colour [none selected]")
    end

    io.Write("\x1b[?2026l")
    io.Flush()
end

module.on_resize = function()
    
    local new_width, new_height = terminal.GetSize()

    if new_width == module.screen_width and new_height == module.screen_height then return end

    module.screen_width = new_width;
    module.screen_height = new_height;

    module.render()
end

module.render_pallete_grid = function()
    local pallete = module.current_pallete

    local grid_start_x = 1
    local grid_start_y = 3

    local grid_cell_size_x = math.max(9,module.screen_width // (#pallete + 1))
    local grid_cell_size_y = 3

    local cell_width = grid_cell_size_x + 1
    local cell_height = grid_cell_size_y + 2

    local columns = math.max(1,(module.screen_width - grid_start_x + 1) // cell_width)

    if columns < #pallete // 2 then 
        for i = 1,#pallete do
            local ansi = pallete[i]:ToAnsi()
            local hex = pallete[i]:ToHex():sub(1,6)

            terminal.MoveCursor(grid_start_x,grid_start_y + (i - 1))
            io.Write(`{ansi}██{style.Reset} {hex}`)
        end

        return
    end

    for i = 1,#pallete do
        local col = (i - 1) % columns
        local row = (i - 1) // columns

        local x = grid_start_x + col * cell_width 
        local y = grid_start_y + row * cell_height

        local cell_red,cell_green,cell_blue = pallete[i]:ToSRGBA()

        local ansi = ("\x1b[48;2;%d;%d;%dm"):format(
            math.floor(math.clamp(cell_red * 255,0,255)),
            math.floor(math.clamp(cell_green * 255,0,255)),
            math.floor(math.clamp(cell_blue * 255,0,255)
        ))

        local hex = pallete[i]:ToHex():sub(1,6)

        module.draw_filled_background_rect(
            x,
            y,
            grid_cell_size_x,
            grid_cell_size_y,
            ansi
        )

        terminal.MoveCursor(x + 1, y + 3)
        io.Write("#" .. hex)
    end
end

module.draw_box = function(x: number,y: number,width: number,height: number)
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

module.draw_filled_rect = function(x: number,y: number,width: number,height: number,colour: string)
    for i = 0,height - 1 do 
        terminal.MoveCursor(x,y + i)
        io.Write(`{colour}{("█"):rep(width)}{style.Reset}`)
    end
    
    io.Flush()
end

module.draw_filled_background_rect = function(x: number,y: number,width: number,height: number,colour: string)
    for i = 0,height - 1 do 
        terminal.MoveCursor(x,y + i)
        io.Write(`{colour}{(" "):rep(width)}{style.Reset}`)
    end

    io.Flush()
end

return module