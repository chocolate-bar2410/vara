--[=[
	@class DateTime

    An Object representing a date and time.
    code example:
    ```luau
    local chrono = require("@vara/chrono")

    local timestamp = chrono.FromUniversal({
        Year = 2025,
        Month = 9,
        Day = 10,

        Hour = 18,
        Minute = 20,
        Second = 0,
        Millisecond = 0,
    })

    print(timestamp)
    ```
]=]
declare extern type DateTime with 
    --- The UTC offset in hours.
    TimeOffset: number
    Year: number
    Month: number
    Day: number
    Hour: number
    Minute: number
    Second: number
    Millisecond: number
    --[=[
		@within DateTime
		@method ToISO8601
        Formats the datetime according to the ISO8601 standard.
    ]=]
    function ToISO8601(self): string
    --[=[
		@within DateTime
		@method ToRFC2822
        Formats the datetime according to the RFC2822 standard.
    ]=]
    function ToRFC2822(self): string
    --[=[
		@within DateTime
		@method Humanise
        Formats the datetime so thats its human readable.
    ]=]
    function Humanise(self): string
    --[=[
		@within DateTime
		@method ToZH
        Formats the datetime so that its human readable in mandarin.
    ]=]
    function ToZH(self): string
    --[=[
		@within DateTime
		@method ToApacheLog
        Formats the datetime as an Apache Log timestamp.
    ]=]
    function ToApacheLog(self): string
    function __tostring(self): string
    function __eq(self,other : DateTime): boolean
    function __le(self,other : DateTime): boolean
    function __lt(self,other : DateTime): boolean

end

type ImageFormat = "PNG" | "JPEG" | "WEBP" | "PNM" | "TIFF" | "BMP" | "QOI"
type ImageResizeFilter = "NEAREST" | "BILINEAR" | "LANCZOS3"


--[=[
	@class Image

    An Object representing an image.
]=]
declare extern type Image with
    read Data: buffer
    read Width: number
    read Height: number
    read Channels: number
    --[=[
        @within Image
        @method Encode
        @param format The image format to encode to
        @return the encoded image data as a buffer.

        Encodes the image into a certain image format.
    ]=]
    function Encode(self,format : ImageFormat) : buffer
    --[=[
        @within Image
        @method Dimensions
        @param width The image width.
        @param height The image height.
        @return itself

        Sets the width and height of the image.
    ]=]
    function Dimensions(self,width: number,height: number) : Image
    --[=[
        @within Image
        @method Resize
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
    function Resize(self,width: number,height: number,filter: ImageResizeFilter?) : Image
    --[=[
        @within Image
        @method Flip
        @param channel Optional parameter to only flip a single colour channel.
        @return itself

        Vertically mirrors the image.
    ]=]
    function Flip(self,channel: number?) : Image
     --[=[
        @within Image
        @method Flop
        @param channel Optional parameter to only flip a single colour channel.
        @return itself

        Horizontally mirrors the image.
    ]=]
    function Flop(self,channel: number?) : Image
    --[=[
        @within Image
        @method Greyscale
        @return itself

        Converts the image into greyscale using luminance.
    ]=]
    function Greyscale(self) : Image
    --[=[
        @within Image
        @method Convolve
        @param width The width of the kernel.
        @param height The height of the kernel.
        @param kernel The kernel applied during the convolution.
        @return itself

        Performs a convolution on the image using the provided kernel.
    ]=]
    function Convolve(self,width : number,height : number,kernel : {number}) : Image
    --[=[
        @within Image
        @method Clone
        @return A clone of your image.

        Clones your image.
    ]=]
    function Clone(self) : Image    
    --[=[
        @within Image
        @method GradientMagnitude
        @param other The other image.
        @return The gradient magnitude between your two images.

        Computes the gradient magnitude between your image and another image.
    ]=]
    function GradientMagnitude(self,other : Image) : Image
    --[=[
        @within Image
        @method RemoveAlpha
        @return itself.

        Removes the alpha channel of your image.
        Treated as a no-op if the image does not have a alpha channel.
    ]=]
    function RemoveAlpha(self) : Image
    function __tostring(self): string
end

type AudioFormat = "WAV" | "FLAC"
type AudioResampleFilter = "HERMITE" | "LINEAR" | "LANCZOS3"
type AudioSaturationMode = "TANH" | "ARCTAN"

type AudioCompressConfig = {
    -- Ammount of gain to add before compressing.
    Input : number?,
    -- Ammount of gain to add after compressing.
    Output : number?,
    -- Volume level (in decibels) to start compressing.
    Threshold : number?,
    -- Ammount to compress by as a ratio.
    Ratio : number?,
    -- How long it takes for the compressor to react.
    Attack : number?,
    -- How long it takes for the compressor to decompress.
    Release : number?,
    -- Softens the transition between uncompressed and compressed.
    Knee : number?
}

--[=[
	@class Audio

    An Object representing an audio.
]=]
declare extern type Audio with
    read Data: {buffer}
    read SampleRate: number
    read Frames: number
    read Duration: number
    read Channels: number
    --[=[
        @within Audio
        @method Encode
        @param format The audio format to encode to
        @return the encoded audio data as a buffer.

        Encodes the audio into a certain audio format.
    ]=]
    function Encode(self,format: AudioFormat): buffer
    --[=[
        @within Audio
        @method Clone
        @return A clone of your audio.

        Clones your audio.
    ]=]
    function Clone(self): Audio  
    --[=[
        @within Audio
        @method Reverse
        @return Itself

        Reverses the audio data.
    ]=]
    function Reverse(self): Audio  
    --[=[
        @within Audio
        @method Amplify
        @param decibel The ammount of gain in decibels
        @return Itself

        Increases the volume by a set gain.
    ]=]
    function Amplify(self,decibel: number): Audio  
    --[=[
        @within Audio
        @method Normalise
        @param decibel The target dBFS of the normalisation.
        @return Itself

        Does a peak normalisation targetting the provided decibel level.
        If no target was provided then it will target 0 dBFS.
    ]=]
    function Normalise(self,decibel: number?): Audio  
    --[=[
        @within Audio
        @method FadeIn
        @param duration The duration of the fade in seconds.
        @return Itself

        Adds a linear fade in that lasts for the given duration.
    ]=]
    function FadeIn(self,duration: number): Audio  
    --[=[
        @within Audio
        @method FadeOut
        @param duration The duration of the fade out seconds.
        @return Itself

        Adds a linear fade out at the end of the audio that lasts for the given duration.
    ]=]
    function FadeOut(self,duration: number): Audio  
    --[=[
        @within Audio
        @method Trim
        @param start The start time in seconds .
        @param stop The stop time in seconds.
        @return Itself

        Trims down the audio to be between a start and a optional stop time.
    ]=]
    function Trim(self,start: number,stop: number?): Audio      
    --[=[
        @within Audio
        @method Lowpass
        @param frequency the cutoff frequency of the filter.
        @param q the Q factor (or 'resonance') that controls the pointiness of the cutoff frequency being boosted.
        @return Itself

        Applies a lowpass filter over the audio.
    ]=]
    function Lowpass(self,frequency: number,q: number?): Audio      
    --[=[
        @within Audio
        @method Highpass
        @param frequency the cutoff frequency of the filter.
        @param q the Q factor (or 'resonance') that controls the pointiness of the cutoff frequency being boosted.
        @return Itself

        Applies a highpass filter over the audio.
    ]=]
    function Highpass(self,frequency: number,q: number?): Audio      
    --[=[
        @within Audio
        @method Notch
        @param frequency the center frequency of the filter.
        @param q the Q factor (or 'resonance') that controls the pointiness of the rejection band around the center frequency.
        @return Itself

        Applies a notch filter over the audio.
    ]=]
    function Notch(self,center_hz: number,q: number?): Audio      
    --[=[
        @within Audio
        @method Resample
        @param sample_rate The new sample count of the audio.
        @param filter The The interpolation filter used when resampling.
        @return Itself

        Resamples the audio to the provided sample rate;
        Interpolation filters supported:

        * HERMITE  - interpolates from 4 points and their gradients.
        * LINEAR   - linear interpolation between 2 points
        * LANCZOS3 - interpolation using 6 points
    ]=]
    function Resample(self,sample_rate: number,filter: AudioResampleFilter): Audio      
    --[=[
        @within Audio
        @method Append
        @param other The audio file being appended.
        @param start Start time (in seconds) to begin slicing the other audio file.
        @param stop  Stop time (in seconds) to stop slicing the other audio file.
        @param crossfade How long should the audios overlap.
        @return Itself

        Appends another audio file to the end of this audio file.
    ]=]
    function Append(self,other: Audio,start: number?,stop: number?,crossfade: number?): Audio 
    --[=[
        @within Audio
        @method Overlay
        @param other The audio file being overlayed.
        @param position Offset to where the other audio is placed.
        @param loop Determines if the other audio should be looped if it stops before the base audio ends.
        @return Itself

        overlays two audio files together.
    ]=]
    function Overlay(self,other: Audio,position: number?,loop : boolean?): Audio 
    --[=[
        @within Audio
        @method HardClip
        @param threshold The max dBFS threshold.
        @return Itself

        Strictly clamps the audio to below the threshold.
    ]=]
    function HardClip(self,threshold: number): Audio 
    --[=[
        @within Audio
        @method SoftClip
        @param threshold The max dBFS threshold.
        @return Itself

        Clamps the audio to below the threshold while preserving peaks.
    ]=]
    function SoftClip(self,threshold: number): Audio 
    --[=[
        @within Audio
        @method Saturate
        @param drive The ammount of gain in decibels applied before softclipping. 
        @param mode The curve of the softclipper.
        @return Itself

        Saturates the audio using a drive parameter.
        modes supported:

        * TANH   - warmer and smoother saturations.
        * ARCTAN - more aggressive saturations.
    ]=]
    function Saturate(self,drive: number,mode : AudioSaturationMode?): Audio 
    --[=[
        @within Audio
        @method Compress
        @param config The configuration for the compressor.

        Compresses the dynamic range of the audio, making quiet sounds louder and loud sounds quieter.
    ]=]
    function Compress(self,config: AudioCompressConfig): Audio 
    function __tostring(self): string
end


--[=[
	@class File

    An Object representing a file.
]=]
declare extern type File with
    read Path: string
    read Size: number
    read Created: DateTime
    read Accessed: DateTime
    read Modified: DateTime
    read Exists: boolean
    read Extension: boolean
    --[=[
        @within File
        @method Read
        @returns Contents of the file.

        Reads the contents of a file as returns it as a buffer.
    ]=]
    function Read(self): buffer
    --[=[
        @within File
        @method Write
        @param contents The contents of the file.
        @return Itself

        Writes the given contents into a file.
    ]=]
    function Write(self,contents : buffer) : File
    --[=[
        @within File
        @method Append
        @param contents The contents of the file.
        @return Itself

        Appends the given contents onto the end of a file.
        If the file does not exist it will instead create a new file to append to.
    ]=]
    function Append(self,contents : buffer) : File
    --[=[
        @within File
        @method Delete

        Deletes the file.
    ]=]
    function Delete(self)
    --[=[
        @within File
        @method Move
        @param path The path to move to.
        @return Itself
        
        Moves the file to a different location.
    ]=]
    function Move(self,path: string) : File
    --[=[
        @within File
        @method Copy
        @param newPath The path to copy to.
        @return The copy
        
        Copies the file to a new location.
    ]=]
    function Copy(self,path: string) : File

    function __tostring(self): string
end

--[=[
	@class Directory

    An Object representing a file directory.
]=]
declare extern type Directory with 
    read Path: string
    read Created: DateTime
    read Accessed: DateTime
    read Modified: DateTime
    read Exists: boolean
    --[=[
        @within Directory
        @method Make
        @return Itself
        
        Creates a file directory where the directory object's path is located.
    ]=]
    function Make(self) : File
    --[=[
        @within Directory
        @method Move
        @param path The path to move to.
        @return Itself
        
        Moves the directory to a different location.
    ]=]
    function Move(self,path: string) : File
    --[=[
        @within Directory
        @method Delete

        Deletes the file directory and all files inside it.
    ]=]
    function Delete(self)
    --[=[
        @within Directory
        @method Walk
        @return iterator

        Returns an non recursive iterator that walks through every file and directory in that directory.
    ]=]
    function Walk(self) : () -> File | Directory

    function __tostring(self): string
end

--[=[
	@class Colour4

    An Object representing a linear rgba colour.
    Colour4 supports HDR, allowing colours channels to be greater than 1.
]=]
declare extern type Colour4 with 
    Red: number
    Green: number
    Blue: number
    Alpha: number

    --[=[
        @within Colour4
        @method ToAnsi
        @return ansi code.

        Returns the true colour ansi code for the corressponding rgb colour.
        the ansi code will be normalised to 0 -> 255.
    ]=]
    function ToAnsi(self): string
    --[=[
        @within Colour4
        @method ToSRGBA
        @return srgba tuple.

        Returns the colour as a tuple of SRGBA values
    ]=]
    function ToSRGBA(self): (number,number,number,number)
    --[=[
        @within Colour4
        @method ToHex
        @return hex string.

        Returns the colour as a hex string
    ]=]
    function ToHex(self): string
    --[=[
        @within Colour4
        @method ToHSVA
        @return hsva tuple.

        Returns the colour as a tuple of HSVA values
    ]=]
    function ToHSVA(self): (number,number,number,number)
    --[=[
        @within Colour4
        @method ToOKLCH
        @return oklch tuple.

        Returns the colour as a tuple of OKLCH values
    ]=]
    function ToOKLCH(self): (number,number,number,number)
    --[=[
        @within Colour4
        @method Luminance
        @return The luminance of the colour.

        Returns the colour as a tuple of OKLCH values
    ]=]
    function Luminance(self): number
    --[=[
        @within Colour4
        @method Greyscale
        @return Itself.

        Converts the colour to greyscale using luminance.
    ]=]
    function Greyscale(self): Colour4
    --[=[
        @within Colour4
        @method Clone
        @return A copy of the Colour4.

        Creates a copy of the existing colour4 object.
    ]=]
    function Clone(self): Colour4
    --[=[
        @within Colour4
        @method Brighten
        @param lightness The ammount to brighten the colour4 by perceptually.
        @return Itself.

        Brightens the colour4 using the OKLAB colour space.
    ]=]
    function Brighten(self,lightness: number): Colour4
    --[=[
        @within Colour4
        @method Darken
        @param lightness The ammount to darken the colour4 by perceptually.
        @return Itself.

        Darkens the colour4 using the OKLAB colour space.
    ]=]
    function Darken(self,lightness: number): Colour4
    --[=[
        @within Colour4
        @method Complement
        @return The complementary colour.

        Finds the colour directly opposite to the base colour on the colour circle.
    ]=]
    function Complement(self): Colour4
    --[=[
        @within Colour4
        @method Triadic
        @return The other 2 triadic colours.

        Finds the exact colours which form a equilateral triangle with the base colour on the colour circle.
    ]=]
    function Triadic(self): (Colour4,Colour4)
    --[=[
        @within Colour4
        @method SplitComplement
        @return The other 2 split complementary colours.

        Finds the colours adjacent to the base colour's complement
    ]=]
    function SplitComplement(self): (Colour4,Colour4)
end

--[=[
    @class _RUNTIME

    _RUNTIME global to follow the [_RUNTIME](https://gist.github.com/Bottersnike/001470cbbb0cd63d9790a542ed5be1bf) spec.

    @prop name The name of the runtime, vara.
    @prop version Version information
    @prop url The github repository for vara.
]=]
declare _RUNTIME: {
    read name: "vara",
    read version: {
        display: string
    },

    read url: string
}