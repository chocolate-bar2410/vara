export type AudioConfigTable = {
    tracks: {buffer},
    sampleRate: number,
    frames: number,
}

--[=[
    @class media
    Built in library for processing media files.

    ### example code
    ```luau
    const media = require("@vara/media")
    const fs = require("@vara/fs")

    local image: Image = media.image("./demo_scripts/images/basn2c16.png")

    fs.write("./demo_scripts/images/basn2c16.qoi",image:encode("QOI"))

    ```
]=]
local module = {}

--[=[
    @within media
    @method image
    @param data A pathstring or a buffer that stores RGBA pixel data.
    @return A decoded image.

    Creates a image from a pathstring or from a buffer.
]=]
module.image = function(data : buffer | string | File) : Image
    return nil :: unknown
end

--[=[
    @within media
    @method audio
    @param data A pathstring or a AudioConfigTable that holds audio data.
    @return A decoded audio object.

    Creates a audio from a pathstring or from a audio config table.
]=]
module.audio = function(data : AudioConfigTable | string | File) : Audio
    return nil :: unknown
end

return module