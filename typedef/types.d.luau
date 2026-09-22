--[=[
	@class DateTime

    An Object representing a date and time.
    code example:
    ```luau
    local chrono = require("@vara/chrono")

    local timestamp = chrono.fromUniversal({
        year = 2025,
        month = 9,
        day = 10,

        hour = 18,
        minute = 20,
        second = 0,
        millisecond = 0,
    })

    print(timestamp)
    ```
]=]
declare extern type DateTime with 
    --- The UTC offset in hours.
    timeOffset: number
    year: number
    month: number
    day: number
    hour: number
    minute: number
    second: number
    millisecond: number
    --[=[
		@within DateTime
		@method toISO8601
        Formats the datetime according to the ISO8601 standard.
    ]=]
    function toISO8601(self): string
    --[=[
		@within DateTime
		@method toRFC2822
        Formats the datetime according to the RFC2822 standard.
    ]=]
    function toRFC2822(self): string
    --[=[
		@within DateTime
		@method humanise
        Formats the datetime so thats its human readable.
    ]=]
    function humanise(self): string
    --[=[
		@within DateTime
		@method toZH
        Formats the datetime so that its human readable in mandarin.
    ]=]
    function toZH(self): string
    --[=[
		@within DateTime
		@method toApacheLog
        Formats the datetime so as an Apache Log timestamp.
    ]=]
    function toApacheLog(self): string
    function __tostring(self): string
    function __eq(self,other : DateTime): boolean
    function __le(self,other : DateTime): boolean
    function __lt(self,other : DateTime): boolean

end

type ImageFormat = "PNG" | "JPEG" | "WEBP" | "PNM" | "TIFF" | "Bmp" | "ICO" | "FARBFELD" | "AVIF" | "QOI"

--[=[
	@class Image

    An Object representing an image.
]=]
declare extern type Image with

    --[=[
        @within Image
        @param format The image format to encode to
        @return the encoded image data as a buffer.

        Encodes the image into a certain image format.
    ]=]
    function encode(self,format : ImageFormat) : buffer

    --[=[
        @within Image
        @param width The image width.
        @param height The image height.
        @return itself

        Sets the width and height of the image.
    ]=]
    function dimensions(self,width: number,height: number) : Image


    function __tostring(self): string
end