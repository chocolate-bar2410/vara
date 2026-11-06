---
icon: octicons/file-directory-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-file-directory-24:</span> 
terminal
</h1>
Vara's library for terminal manipulation.

---

## fields

<h3 markdown>Style<span class="API-type"> : {[string]: string} </span></h3>
ANSI escape codes for styling text.

<h3 markdown>Colour<span class="API-type"> : {[string]: string} </span></h3>
ANSI escape codes for coloured text.

## functions

<h3 markdown>Clear<span class="API-type"> -> () </span></h3>
```luau
terminal.Clear()
```
Clears the terminal

<h3 markdown>HideCursor<span class="API-type"> -> () </span></h3>
```luau
terminal.HideCursor()
```
Hides the cursor.

<h3 markdown>ShowCursor<span class="API-type"> -> () </span></h3>
```luau
terminal.ShowCursor()
```
Shows the cursor.

<h3 markdown>MoveCursor<span class="API-type"> -> () </span></h3>
```luau
terminal.MoveCursor(
    x: number,
    y: number
)
```
Shows the cursor.

<h3 markdown>EnterAltScreen<span class="API-type"> -> () </span></h3>
```luau
terminal.EnterAltScreen()
```
Moves to an alternate screen buffer.

<h3 markdown>ExitAltScreen<span class="API-type"> -> () </span></h3>
```luau
terminal.ExitAltScreen()
```
Exits an alternate screen buffer.

<h3 markdown>EnterRawMode<span class="API-type"> -> () </span></h3>
```luau
terminal.EnterRawMode()
```
Sets the terminal to raw mode.

<h3 markdown>ExitRawMode<span class="API-type"> -> () </span></h3>
```luau
terminal.ExitRawMode()
```
Makes the terminal exit raw mode.

<h3 markdown>GetSize<span class="API-type"> -> (number,number) </span></h3>
```luau
terminal.GetSize(): (number,number)
```
Gets the size of the terminal.
