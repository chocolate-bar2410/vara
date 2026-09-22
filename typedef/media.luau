--[=[
    @class media
    Built in library for processing media files.

    ### example code
    ```luau
    const media = require("@vara/media")
    const fs = require("@vara/fs")

    local data: buffer = fs.read("./demo_scripts/images/basn2c16.png")
    local image: Image = media.decodeImage("PNM",data)

    fs.write("./demo_scripts/images/basn2c16.qoi",image:encode("QOI"))

    ```
]=]
local module = {}

type ImageFormat = "PNG" | "JPEG" | "WEBP" | "PNM" | "TIFF" | "Bmp" | "ICO" | "FARBFELD" | "AVIF" | "QOI"

--[=[
    @within media
    @method decodeImage
    @param imageFormat The image format of the image.
    @param data The image data as a buffer.
    @return A decoded image.

    Decodes image data into a image object.
]=]
module.decodeImage = function(imageFormat : ImageFormat,data : buffer) : Image
    return nil :: unknown
end

return module