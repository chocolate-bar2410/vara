---
icon: octicons/unmute-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-unmute-24:</span> 
Audio
</h1>

An Object representing an audio.  

---

## properties
<h3 markdown>Data<span class="API-type"> : {buffer}</span></h3>

The data represented as a table of 32bit float buffers.  
Each buffer represents 1 audio channel.

<h3 markdown>SampleRate<span class="API-type"> : number</span></h3>

The sample rate in hz.

<h3 markdown>Frames<span class="API-type"> : number</span></h3>

The ammount of frames per channel.

<h3 markdown>Duration<span class="API-type"> : number</span></h3>

The duration of the audio in seconds.

<h3 markdown>Channels<span class="API-type"> : number</span></h3>

The ammount of audio channels in the audio.

---

## methods

<h3 markdown>Encode<span class="API-type"> -> buffer </span></h3>
```luau
Image:Encode(format: AudioFormat): buffer
```

Encodes the audio into a certain audio format.

<h3 markdown>Clone<span class="API-type"> -> Audio </span></h3>
```luau
Image:Clone(self): Audio  
```

Clones your audio.

<h3 markdown>Reverse<span class="API-type"> -> Audio </span></h3>
```luau
Image:Reverse(self): Audio  
```

Reverses the audio data.

<h3 markdown>Amplify<span class="API-type"> -> Audio </span></h3>
```luau
Image:Amplify(decibel: number): Audio  
```

Increases the volume by a set gain.

<h3 markdown>Normalise<span class="API-type"> -> Audio </span></h3>
```luau
Image:Normalise(decibel: number?): Audio  
```

Does a peak normalisation targetting the provided decibel level.
        If no target was provided then it will target 0 dBFS.

<h3 markdown>FadeIn<span class="API-type"> -> Audio </span></h3>
```luau
Image:FadeIn(duration: number): Audio  
```

Adds a linear fade in that lasts for the given duration.

<h3 markdown>FadeOut<span class="API-type"> -> Audio </span></h3>
```luau
Image:FadeOut(duration: number): Audio  
```

Adds a linear fade out at the end of the audio that lasts for the given duration.

<h3 markdown>Trim<span class="API-type"> -> Audio </span></h3>
```luau
Image:Trim(
    start: number,
    stop: number?
): Audio      
```

Trims down the audio to be between a start and a optional stop time.

<h3 markdown>Lowpass<span class="API-type"> -> Audio </span></h3>
```luau
Image:Lowpass(
    frequency: number,
    q: number?
): Audio      
```

Applies a lowpass filter over the audio.

<h3 markdown>Highpass<span class="API-type"> -> Audio </span></h3>
```luau
Image:Highpass(
    frequency: number,
    q: number?
): Audio      
```

Applies a highpass filter over the audio.

<h3 markdown>Notch<span class="API-type"> -> Audio </span></h3>
```luau
Image:Notch(
    center_hz: number,
    q: number?
): Audio      
```

Applies a notch filter over the audio.

<h3 markdown>Resample<span class="API-type"> -> Audio </span></h3>
```luau
Image:Resample(
    sample_rate: number,
    filter: AudioResampleFilter
): Audio      
```

Resamples the audio to the provided sample rate;
Interpolation filters supported:

* HERMITE  - interpolates from 4 points and their gradients.
* LINEAR   - linear interpolation between 2 points
* LANCZOS3 - interpolation using 6 points

<h3 markdown>Append<span class="API-type"> -> Audio </span></h3>
```luau
Image:Append(
    other: Audio,
    start: number?,
    stop: number?,
    crossfade: number?
): Audio 
```

Appends another audio file to the end of this audio file.

<h3 markdown>Overlay<span class="API-type"> -> Audio </span></h3>
```luau
Image:Overlay(
    other: Audio,
    position: number?,
    loop : boolean?
): Audio 
```

overlays two audio files together.

<h3 markdown>HardClip<span class="API-type"> -> Audio </span></h3>
```luau
Image:HardClip(threshold: number): Audio 
```

Strictly clamps the audio to below the threshold.

<h3 markdown>SoftClip<span class="API-type"> -> Audio </span></h3>
```luau
Image:SoftClip(threshold: number): Audio 
```

Clamps the audio to below the threshold while preserving peaks.

<h3 markdown>Saturate<span class="API-type"> -> Audio </span></h3>
```luau
Image:Saturate(
    drive: number,
    mode : AudioSaturationMode?
): Audio 
```

Saturates the audio using a drive parameter.
modes supported:

* TANH   - warmer and smoother saturations.
* ARCTAN - more aggressive saturations.

<h3 markdown>Compress<span class="API-type"> -> Audio </span></h3>
```luau
Image:Compress(config: AudioCompressConfig): Audio 
```

Compresses the dynamic range of the audio, making quiet sounds louder and loud sounds quieter.

---

## metamethods

<h3 markdown>__tostring<span class="API-type"> -> string </span></h3>

Converts the audio into a string in the format `audio [Channels]ch [SampleRate]Hz [Frames] frames`

---

## types

<h3 markdown>AudioFormat</h3>

```luau
type AudioFormat = "WAV" | "FLAC"
```
<h3 markdown>AudioResampleFilter</h3>

```luau
type AudioResampleFilter = "HERMITE" | "LINEAR" | "LANCZOS3"
```
<h3 markdown>AudioSaturationMode</h3>

```luau
type AudioSaturationMode = "TANH" | "ARCTAN"
```

<h3 markdown>AudioCompressConfig</h3>

```luau
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
```