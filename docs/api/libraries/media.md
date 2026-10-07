---
title: media
---

Vara's library for processing media files.

```luau
const media = require("@vara/media")
const fs = require("@vara/fs")

local image: Image = media.Image("./demo_scripts/images/basn2c16.png")

fs.Write("./demo_scripts/images/basn2c16.qoi",image:Encode("QOI"))

```

## functions

<h3 markdown>Image<span class="API-type"> -> Image </span></h3>
```luau
media.Image = function(
    data : buffer | string | File
) : Image
```
Creates a image from a File, pathstring or a raw buffer.  
Creating a image from a raw buffer assumes the image is RGBA.

<h3 markdown>Audio<span class="API-type"> -> Audio </span></h3>
```luau
media.Audio = function(
    data : AudioConfigTable | string | File
) : Audio
```
Creates a audio from a File, pathstring or an audio config table.


<h3 markdown>AudioConfigTable</h3>

```luau
type AudioConfigTable = {
    -- tracks containing a 32bit float buffer per channel.
    Tracks: {buffer},
    -- the sample rate in hz.
    SampleRate: number,
    -- the ammount of frames.
    Frames: number,
}
```