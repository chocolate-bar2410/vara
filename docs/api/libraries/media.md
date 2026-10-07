---
title: media
---

Vara's library for manipulating media files.

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