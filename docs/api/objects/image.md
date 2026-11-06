---
icon: octicons/image-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-image-24:</span> 
Image 
</h1>

An object representing a image.  
Images support HDR colour, so values can range from 0 - ∞

---

## properties

<h3 markdown>Data<span class="API-type"> : buffer</span></h3>

The image data as a unsigned 32 bit float interleaved buffer.  

<h3 markdown>Width<span class="API-type"> : number</span></h3>

The width of the image in pixels.  

<h3 markdown>Height<span class="API-type"> : number</span></h3>

The height of the image in pixels.  

<h3 markdown>Channels<span class="API-type"> : number</span></h3>

How many colour channels the image will be exported with.

---

## methods

<h3 markdown>Encode<span class="API-type"> -> buffer </span></h3>
```luau
Image:Encode(format : ImageFormat) : buffer
```

Encodes the image into a certain image format.  

<h3 markdown>Dimensions<span class="API-type"> -> Image </span></h3>
```luau
Image:Dimensions(
    width: number,
    height: number
) : Image
```

Sets the width and height of the image.  
Returns itself.  

<h3 markdown>Resize<span class="API-type"> -> Image </span></h3>
```luau
Image:Resize(
    width: number,
    height: number,
    filter: ImageResizeFilter?
) : Image
```

Resizes the image to a new width & height.  
Interpolation filters supported:
  
* **NEAREST**  - nearest neighbour
* **BILINEAR** - bilinear interpolation between 4 points
* **LANCZOS3** - LANCZOS interpolation using 36 points

Returns itself.

<h3 markdown>Flip<span class="API-type"> -> Image </span></h3>
```luau
Image:Flip(channel: number?) : Image
```

Vertically mirrors the image.  
Returns itself.  

<h3 markdown>Flop<span class="API-type"> -> Image </span></h3>
```luau
Image:Flop(channel: number?) : Image
```

Horizontally mirrors the image.  
Returns itself.  

<h3 markdown>Greyscale<span class="API-type"> -> Image </span></h3>
```luau
Greyscale() : Image
```

Converts the image into greyscale using luminance.  
Returns itself.  

<h3 markdown>Convolve<span class="API-type"> -> Image </span></h3>
```luau
Image:Convolve(
    width : number,
    height : number,
    kernel : {number}
) : Image
```

Performs a convolution on the image using the provided kernel.  
Returns itself.  

<h3 markdown>Clone<span class="API-type"> -> Image </span></h3>
```luau
Clone() : Image    
```

Clones the image, returning its copy.

<h3 markdown>GradientMagnitude<span class="API-type"> -> Image </span></h3>
```luau
Image:GradientMagnitude(other : Image) : Image
```

Computes the gradient magnitude between the image and another image.  
Returns itself.  

<h3 markdown>RemoveAlpha<span class="API-type"> -> Image </span></h3>
```luau
Image:RemoveAlpha() : Image
```

Removes the alpha channel of the image.  
Treated as a no-op if the image does not have a alpha channel.  
Returns itself.  

<h3 markdown>GetPixel<span class="API-type"> -> Colour4 </span></h3>
```luau
Image:GetPixel(
    x: number,
    y: number
) : Colour4
```
Gets the pixel colour at the position (x,y)

<h3 markdown>SetPixel<span class="API-type"> -> Colour4 </span></h3>
```luau
Image:SetPixel(
    x: number,
    y: number,
    colour: Colour4
)
```
Sets the pixel colour at the position (x,y) to the given colour.

<h3 markdown>HdrTonemap<span class="API-type"> -> Image </span></h3>
```luau
Image:HdrTonemap(config: ImageHdrConfig) : Image
```
Performs a hdr tonemapping algorithm on the image.
This reduces the dynamic range from 0 - ∞ to 0 - 1.

Filters supported:  

* **REINHARD** - Map to a asymtopic curve.  
* **ACES** - Map to a ACES filmic curve  
* **HABLE** - Modelled after the response of analog film.  
* **UCHIMURA** - Map to a piecewise curve.  

Returns itself.  

---

## metamethods

<h3 markdown>__tostring<span class="API-type"> -> string </span></h3>

Converts the image into a string in the format `image [Width]x[Height]`

---

## types

<h3 markdown>ImageFormat</h3>
```luau
type ImageFormat = "PNG" | "JPEG" | "WEBP" | "PNM" | "TIFF" | "BMP" | "QOI" | "HDR" | "EXR"
```

<h3 markdown>ImageResizeFilter</h3>
```luau
type ImageResizeFilter = "NEAREST" | "BILINEAR" | "LANCZOS3"
```

<h3 markdown>ImageHdrConfig</h3>
```luau
type ImageHdrConfig = {
    Filter: "REINHARD",
    Exposure: number?
} | {
    Filter: "ACES",
    Exposure: number?
} | {
    Filter: "HABLE",
    ShoulderStrength: number?,
    LinearStrength: number?,
    LinearAngle: number?,
    ToeStrength: number?,
    ToeNumerator: number?,
    ToeDenominator: number?,

    Exposure: number?,
    WhitePoint: number?
} | {
    Filter: "UCHIMURA",
    Contrast: number?,
    LinearSection: number?,
    LinearLength: number?,
    Toe: number?,
    Pedestal: number?,

    Exposure: number?,
    WhitePoint: number?
}
```