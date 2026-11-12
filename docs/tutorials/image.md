---
title: Images
---

Vara comes with the ability to process images using its [image](../api/objects/image.md) object.
Images in vara allow you to chain multiple transformations together, creating a pipeline of transforms.

## Creating a image

A image can either be created from a buffer or by decoding a image file.

=== "decoding"
    ```luau linenums="1"
    const media = require("@vara/media")

    local image = media.Image("example.png")
    ```
=== "from buffer"
    ```luau linenums="1"
    const media = require("@vara/media")
    const HEIGHT = 512
    const WIDTH = 512
    const CC = 4
    const BITDEPTH = 4

    local source = buffer.create(HEIGHT * WIDTH * CC * BITDEPTH)
    for i = 0,HEIGHT * WIDTH * CC - 1,1 do
        buffer.writef32(source,i * BITDEPTH,1)
    end

    local image = media.Image(source):Dimensions(WIDTH,HEIGHT)
    ```

Images store pixel data as interleaved, linear RGBA 32 bit floating point values. Each pixel consists of a red, green, blue and alpha channels.

To export this image, we first encode it to a set format, then save it as a file.

```luau linenums="1"
const fs = require("@vara/fs")

fs.File("output.png"):Write(image:Encode("PNG"))
```

## Transformations

Multiple transformations can be applied on the same image as a chain, unless stated otherwise the method will mutate the image and return itself.

```luau linenums="1"
const media = require("@vara/media")

local image = media.Image("example.png")

-- this mutates the existing image, no new image is created.
image:Flip()
    :Flop()
    :Greyscale()
    :Resize(1024,1024,"BILINEAR")
```

## HDR

While the min and max pixel values that can be exported are 0 - 1, this doesn't mean values greater than 1 aren't supported.  
In reality values can range from 0 - ∞. This allows for a `High Dynamic Range` between the darkest and brightest part of a image, hence HDR.  

Fields such as photography, image processing and graphics programming rely on HDR values preserve light intensity and contrast in their images.  

If you tried to export a HDR image directly, unless you export as a .hdr or .exr, the values will be clamped to 0 - 1.  
This cause your image to become overexposed and loose information.  

We can solve this using tone mapping, this applies a curve that maps our image to a 0 - 1 range.
Unlike clamping the image, this naturally adjusts our image to our required range.

```luau linenums="1"
const media = require("@vara/media")
local fs = require("@vara/fs")

local image = media.Image("example.hdr")
local output = fs.File("output.png")

image:HdrTonemap({
    Filter = "ACES",
    Exposure = -1
})

output:Write(image:Encode("PNG"))
```