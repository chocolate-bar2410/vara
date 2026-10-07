---
title: chrono
---

Vara's library for handling date times.

!!! danger
    functions relating to timezones only work on windows, timezones for unix systems have not been implemented yet

## functions

<h3 markdown> Now <span class="API-type"> -> DateTime </span></h3>
```luau
chrono.Now() : DateTime
```
Gets the current date and time according adjusted to the local timezone.

<h3 markdown> FromUnix <span class="API-type"> -> DateTime </span></h3>
```luau
chrono.FromUnix(unixTime : number) : DateTime
```
Converts a unix timestamp into a DateTime.

<h3 markdown> FromUniversal <span class="API-type"> -> DateTime </span></h3>
```luau
chrono.FromUniversal(params : DateTimeParameters) : DateTime
```
Constructs a DateTime using a table of parameters at UTC+00.

<h3 markdown> FromLocal <span class="API-type"> -> DateTime </span></h3>
```luau
chrono.FromLocal(params : DateTimeParameters) : DateTime
```
Constructs a DateTime using a table of parameters in the local timezone.

<h3 markdown> TimezoneOffset <span class="API-type"> -> number </span></h3>
```luau
chrono.TimezoneOffset() : number
```
returns the current timezone offset in hours.

## types

<h3 markdown>DateTimeParameters</h3>

```luau
type DateTimeParameters = {
    Year : number,
    Month : number,
    Day : number,

    Hour : number,
    Minute : number,
    Second : number,
    Millisecond : number?,
}
```