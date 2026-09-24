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
type ImageResizeFilter = "NEAREST" | "BILINEAR" | "LANCZOS3"


--[=[
	@class Image

    An Object representing an image.
]=]
declare extern type Image with
    read data: buffer
    read width: number
    read height: number
    read channels: number
    --[=[
        @within Image
        @method encode
        @param format The image format to encode to
        @return the encoded image data as a buffer.

        Encodes the image into a certain image format.
    ]=]
    function encode(self,format : ImageFormat) : buffer
    --[=[
        @within Image
        @method dimensions
        @param width The image width.
        @param height The image height.
        @return itself

        Sets the width and height of the image.
    ]=]
    function dimensions(self,width: number,height: number) : Image
    --[=[
        @within Image
        @method resize
        @param width The new image width.
        @param height The new image height.
        @param filter The interpolation filter being ran.
        @return itself

        Resizes the image to a new width & height.
        Interpolation filters supported:

        * NEAREST  - nearest neighbour
        * BILINEAR - bilinear interpolation between 4 points
        * LANCZOS3 - LANCZOS interpolation using 36 points
    ]=]
    function resize(self,width: number,height: number,filter: ImageResizeFilter?) : Image
    --[=[
        @within Image
        @method flip
        @param channel Optional parameter to only flip a single colour channel.
        @return itself

        Vertically mirrors the image.
    ]=]
    function flip(self,channel: number?) : Image
     --[=[
        @within Image
        @method flop
        @param channel Optional parameter to only flip a single colour channel.
        @return itself

        Horizontally mirrors the image.
    ]=]
    function flop(self,channel: number?) : Image
    --[=[
        @within Image
        @method greyscale
        @return itself

        Converts the image into greyscale using luminance.
    ]=]
    function greyscale(self) : Image
    --[=[
        @within Image
        @method convolve
        @param width The width of the kernel.
        @param height The height of the kernel.
        @param kernel The kernel applied during the convolution.
        @return itself

        Performs a convolution on the image using the provided kernel.
    ]=]
    function convolve(self,width : number,height : number,kernel : {number}) : Image
    --[=[
        @within Image
        @method clone
        @return A clone of your image.

        Clones your image.
    ]=]
    function clone(self) : Image    
    --[=[
        @within Image
        @method gradientMagnitude
        @param other The other image.
        @return The gradient magnitude between your two images.

        Computes the gradient magnitude between your image and another image.
    ]=]
    function gradientMagnitude(self,other : Image) : Image
    --[=[
        @within Image
        @method removeAlpha
        @return itself.

        Removes the alpha channel of your image.
        Treated as a no-op if the image does not have a alpha channel.
    ]=]
    function removeAlpha(self) : Image
    function __tostring(self): string
end

type AudioFormat = "WAV" | "FLAC"

--[=[
	@class Audio

    An Object representing an audio.
]=]
declare extern type Audio with
    data: {buffer}
    sample_rate: number
    frames: number
    duration: number
    channels: number
    --[=[
        @within Audio
        @method encode
        @param format The audio format to encode to
        @return the encoded audio data as a buffer.

        Encodes the audio into a certain audio format.
    ]=]
    function encode(self,format: AudioFormat): buffer
    --[=[
        @within Audio
        @method clone
        @return A clone of your audio.

        Clones your audio.
    ]=]
    function clone(self): Audio  

    --[=[
        @within Audio
        @method reverse
        @return Itself

        Reverses the audio data.
    ]=]
    function reverse(self): Audio  
    --[=[
        @within Audio
        @method normalise
        @param decibel The ammount of gain in decibels
        @return Itself

        Does a peak normalisation targetting the provided decibel level.
        If no target was provided then it will target 0 dbfs.
    ]=]
    function normalise(self,decibel: number?): Audio  

    function __tostring(self): string
end