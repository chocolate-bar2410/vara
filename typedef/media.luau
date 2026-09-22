--[=[
    @class media
    Built in library for processing media files.
]=]
local module = {}

type ImageFormat = "PNG" | "JPEG" | "WEBP" | "PNM" | "TIFF" | "Bmp" | "ICO" | "FARBFELD" | "AVIF" | "QOI"

module.decodeImage = function(fileFormat : ImageFormat,data : buffer) : Image
    return nil :: unknown
end

return module