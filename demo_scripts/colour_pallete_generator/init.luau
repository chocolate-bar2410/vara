local io = require("@vara/io")

local palletes = require("@self/palletes")
local tui = require("@self/tui")

tui.init()
tui.render()

while true do
    local key = io.ReadAsync(1)

    if key == string.char(3) then break end

    if key == string.char(32) then
        local pallete = palletes.generate_pallete()
        tui.current_pallete = pallete
        tui.pallete_string = palletes.colour_bar(pallete)

        tui.render()
    end
end

tui.exit()

