---
icon: octicons/calendar-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-calendar-24:</span> 
Colour4
</h1>

An Object representing a linear rgba colour.  
Colour4 supports HDR, allowing colours channels to be greater than 1.

---

## properties

<h3 markdown>TimeOffset<span class="API-type"> : number</span></h3>

The UTC offset in hours.

<h3 markdown>Red<span class="API-type"> : number</span></h3>
The Red channel of the colour.

<h3 markdown>Green<span class="API-type"> : number</span></h3>
The Green channel of the colour.

<h3 markdown>Blue<span class="API-type"> : number</span></h3>
The Blue channel of the colour.

<h3 markdown>Alpha<span class="API-type"> : number</span></h3>
The Alpha/Transparency channel of the colour.

---

## methods

<h3 markdown>ToAnsi<span class="API-type"> -> string </span></h3>
```luau
Colour4:ToAnsi(): string
```
Returns the true colour ansi code for the corressponding rgb colour.
        the ansi code will be normalised to 0 -> 255.

<h3 markdown>ToSRGBA<span class="API-type"> -> (number,number,number,number) </span></h3>
```luau
Colour4:ToSRGBA(): (number,number,number,number)
```
Returns the colour as a tuple of SRGBA values

<h3 markdown>ToHex<span class="API-type"> -> string </span></h3>
```luau
Colour4:ToHex(): string
```
Returns the colour as a hex string

<h3 markdown>ToHSVA<span class="API-type"> -> (number,number,number,number) </span></h3>
```luau
Colour4:ToHSVA(): (number,number,number,number)
```
Returns the colour as a tuple of HSVA values

<h3 markdown>ToOKLCH<span class="API-type"> -> (number,number,number,number) </span></h3>
```luau
Colour4:ToOKLCH(): (number,number,number,number)
```
Returns the colour as a tuple of OKLCH values

<h3 markdown>Luminance<span class="API-type"> -> number </span></h3>
```luau
Colour4:Luminance(): number
```
Returns the colour as a tuple of OKLCH values

<h3 markdown>Greyscale<span class="API-type"> -> Colour4 </span></h3>
```luau
Colour4:Greyscale(): Colour4
```
Converts the colour to greyscale using luminance.
Returns itself.  

<h3 markdown>Clone<span class="API-type"> -> Colour4 </span></h3>
```luau
Colour4:Clone(): Colour4
```
Creates a copy of the existing colour4 object.

<h3 markdown>Brighten<span class="API-type"> -> Colour4 </span></h3>
```luau
Colour4:Brighten(lightness: number): Colour4
```
Brightens the colour4 using the OKLAB colour space.  
Returns itself.  

<h3 markdown>Darken<span class="API-type"> -> Colour4 </span></h3>
```luau
Colour4:Darken(lightness: number): Colour4
```
Darkens the colour4 using the OKLAB colour space.
Returns itself.  

<h3 markdown>Complement<span class="API-type"> -> Colour4 </span></h3>
```luau
Colour4:Complement(): Colour4
```
Finds the colour directly opposite to the base colour on the colour circle.

<h3 markdown>Triadic<span class="API-type"> -> (Colour4,Colour4) </span></h3>
```luau
Colour4:Triadic(): (Colour4,Colour4)
```
Finds the exact colours which form a equilateral triangle with the base colour on the colour circle.

<h3 markdown>SplitComplement<span class="API-type"> -> (Colour4,Colour4) </span></h3>
```luau
Colour4:SplitComplement(): (Colour4,Colour4)
```
Finds the colours adjacent to the base colour's complement

<h3 markdown>HdrTonemap<span class="API-type"> -> Colour4 </span></h3>
```luau
Colour4:HdrTonemap(): Colour4
```
Applies a reinhard HDR tonemap filter to normalise colour channels to 0 -> 1.


---

## metamethods

<h3 markdown>__tostring<span class="API-type"> -> string </span></h3>

Converts the image into a string in the format `colour4 [Red] [Green] [Blue] [Alpha]`