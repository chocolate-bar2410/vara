export type AudioConfigTable = {
    Tracks: {buffer},
    SampleRate: number,
    Frames: number,
}

--[=[
    @class media
    Built in library for processing media files.

    ### example code
    ```luau
    const media = require("@vara/media")
    const fs = require("@vara/fs")

    local image: Image = media.Image("./demo_scripts/images/basn2c16.png")

    fs.Write("./demo_scripts/images/basn2c16.qoi",image:Encode("QOI"))

    ```
]=]
local module = {}

--[=[
    @within media
    @method Image
    @param data A pathstring or a buffer that stores RGBA pixel data.
    @return A decoded image.

    Creates a image from a pathstring or from a buffer.
]=]
module.Image = function(data : buffer | string | File) : Image
    return nil :: unknown
end

--[=[
    @within media
    @method Audio
    @param data A pathstring or a AudioConfigTable that holds audio data.
    @return A decoded audio object.

    Creates a audio from a pathstring or from a audio config table.
]=]
module.Audio = function(data : AudioConfigTable | string | File) : Audio
    return nil :: unknown
end

return module