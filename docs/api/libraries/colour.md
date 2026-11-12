---
icon: octicons/paintbrush-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-paintbrush-24:</span> 
colour
</h1>
Vara's library for creating colours.

---

## functions

<h3 markdown>new<span class="API-type"> -> colour4 </span></h3>
```luau
colour.New(
    red: number,
    green: number,
    blue: number,
    alpha: number?
) : Colour4
```
Creates a new Colour4.

<h3 markdown>FromSRGBA<span class="API-type"> -> colour4 </span></h3>
```luau
colour.FromSRGBA(
    red: number,
    green: number,
    blue: number,
    alpha: number?
) : Colour4
```
Creates a new Colour4 from the given srgba values.

<h3 markdown>FromHex<span class="API-type"> -> colour4 </span></h3>
```luau
colour.FromHex(hex_string: string) : Colour4
```
Creates a new Colour4 from a hex string.

<h3 markdown>FromHSVA<span class="API-type"> -> colour4 </span></h3>
```luau
colour.FromHSVA(
    hue: number,
    saturation: number,
    value: number,
    alpha: number?
) : Colour4
```
Creates a new Colour4 from a hue, saturation and value.

<h3 markdown>FromOKLCH<span class="API-type"> -> colour4 </span></h3>
```luau
colour.FromOKLCH(
    lightness: number,
    chroma: number,
    hue: number,
    alpha: number?
) : Colour4
```    
Creates a new Colour4 from the perceptually even OKLCH colourspace, which is derived from OKLAB.
