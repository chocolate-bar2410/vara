---
title: serde
---

Vara's library for serialising and deserialising data.

```luau
local serde = require("@vara/serde")

local message = serde.Encode("JSON", { hello = "world", testing = 123 })
print(message)

local data : { hello: string, testing: number } = serde.Decode("JSON",message)
print(data.hello)
print(data.testing)
```

## functions

<h3 markdown>Encode<span class="API-type"> -> string </span></h3>
```luau
serde.Encode(
    format: SerdeFormat, 
    data: unknown
): string
```
Encodes the data provided into a specified file format.

<h3 markdown>Decode<span class="API-type"> -> unknown </span></h3>
```luau
serde.Decode(
    format: SerdeFormat, 
    data: string
): unknown
```
Decode the data provided from a specified file format into luau values.

## types
<h3 markdown>SerdeFormat</h3>

```luau
type SerdeFormat = "JSON" | "TOML" | "YAML"
```