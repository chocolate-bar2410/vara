---
icon: octicons/paste-16
---

<h1 class="API-header" markdown> 
<span class="API-icon" markdown>:octicons-paste-24:</span> 
Clipboard
</h1>

An object that accesses the system's clipboard.

---

## methods

<h3 markdown>GetText<span class="API-type"> -> string? </span></h3>
```luau
Clipboard:GetText(): string?
```
Gets text data from the user's clipboard.

<h3 markdown>SetText<span class="API-type"> -> () </span></h3>
```luau
Clipboard:SetText(text: string)
```
Places text onto the clipboard

<h3 markdown>GetImage<span class="API-type"> -> Image </span></h3>
```luau
Clipboard:GetImage(): Image
```
Gets image data from the clipboard.

<h3 markdown>SetImage<span class="API-type"> -> () </span></h3>
```luau
Clipboard:SetImage(image: Image)
```
Places an image onto the clipboard

