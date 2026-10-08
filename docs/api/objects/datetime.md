---
icon: octicons/calendar-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-calendar-24:</span> 
DateTime
</h1>

An Object representing a date and time.

```luau
local chrono = require("@vara/chrono")

local timestamp = chrono.FromUniversal({
    Year = 2025,
    Month = 9,
    Day = 10,

    Hour = 18,
    Minute = 20,
    Second = 0,
    Millisecond = 0,
})

print(timestamp:Humanise())
```

## properties

<h3 markdown>TimeOffset<span class="API-type"> : number</span></h3>

The UTC offset in hours.

<h3 markdown>Year<span class="API-type"> : number</span></h3>

The ammount of years.

<h3 markdown>Month<span class="API-type"> : number</span></h3>

The ammount of months.

<h3 markdown>Day<span class="API-type"> : number</span></h3>

The ammount of Days.

<h3 markdown>Hour<span class="API-type"> : number</span></h3>

The ammount of Hours.

<h3 markdown>Minute<span class="API-type"> : number</span></h3>

The ammount of Minutes.

<h3 markdown>Second<span class="API-type"> : number</span></h3>

The ammount of Seconds.

<h3 markdown>Millisecond<span class="API-type"> : number</span></h3>

The ammount of Milliseconds.

## methods

<h3 markdown>ToISO8601<span class="API-type"> -> string </span></h3>
```luau
DateTime:ToISO8601(self): string
```

Formats the datetime according to the ISO8601 standard.

<h3 markdown>ToRFC2822<span class="API-type"> -> string </span></h3>
```luau
DateTime:ToRFC2822(self): string
```

Formats the datetime according to the RFC2822 standard.

<h3 markdown>Humanise<span class="API-type"> -> string </span></h3>
```luau
DateTime:Humanise(self): string
```

Formats the datetime so thats its human readable.

<h3 markdown>ToZH<span class="API-type"> -> string </span></h3>
```luau
DateTime:ToZH(self): string
```

Formats the datetime so that its human readable in mandarin.

<h3 markdown>toApacheLog<span class="API-type"> -> string </span></h3>
```luau
DateTime:ToApacheLog(self): string
```

Formats the datetime as an Apache Log timestamp.


## metamethods

<h3 markdown>__tostring<span class="API-type"> -> string </span></h3>
```luau
function __tostring(): string
```

Converts the data to a string, formatted according to ToISO8601.

<h3 markdown>__eq<span class="API-type"> -> boolean </span></h3>
```luau
function __eq(other : DateTime): boolean
```

Checks if 2 datetimes are equivalent.

<h3 markdown>__le<span class="API-type"> -> boolean </span></h3>
```luau
function __le(other : DateTime): boolean
```

Checks if the datetime is earlier than or equivalent to another datetime.

<h3 markdown>__lt<span class="API-type"> -> boolean </span></h3>
```luau
function __lt(other : DateTime): boolean
```

Checks if the datetime is earlier than another datetime.