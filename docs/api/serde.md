---
title: serde
---

Vara's library for serialising and deserialising data.

## functions

<h3 markdown>Encode<span class="API-type"> -> string </span></h3>
```luau
serde.Encode(
    format: "JSON" | "TOML" | "YAML", 
    data: unknown
): string
```
Encodes the data provided into a specified file format.

<h3 markdown>Decode<span class="API-type"> -> unknown </span></h3>
```luau
serde.Decode(
    format: "JSON" | "TOML" | "YAML", 
    data: string
): unknown
```
Decode the data provided from a specified file format into luau values.