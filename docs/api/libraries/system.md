---
title: system
---

Vara's library for accessing system data.

## fields
<h3 markdown>Os<span class="API-type"> : "linux" | "macos" | "windows" </span></h3>

The operating system being used.

<h3 markdown>Arch<span class="API-type"> : "x86_64" | "aarch64" </span></h3>

The architecture of the processor thats currently being used.

<h3 markdown>Family<span class="API-type"> : "unix" | "windows" </span></h3>

The family of operating system being used.

<h3 markdown>Cwd<span class="API-type"> : string </span></h3>

The current working directory.

<h3 markdown>Temp<span class="API-type"> : string </span></h3>

The temp directory.

<h3 markdown>Home<span class="API-type"> : string </span></h3>

The home directory.

<h3 markdown>Endian<span class="API-type"> : "big" | "little" </span></h3>

The endianess of the processor being used.

<h3 markdown>LanguagePreferences<span class="API-type"> : LanguagePreferences </span></h3>

Language preferences set on the user's system.

<h3 markdown>Distro<span class="API-type"> : string </span></h3>

What operating system distribution the system is using.

<h3 markdown>Device<span class="API-type"> : string </span></h3>

The name of the device currently being used.

<h3 markdown>ProcessID<span class="API-type"> : number </span></h3>

The PID of the current process.

<h3 markdown>DesktopEnvironment<span class="API-type"> : number </span></h3>

The desktop environment the system is using.

<h3 markdown>Env<span class="API-type"> : {[string] : string} </span></h3>

The environment variables declared by the system and a optional .env file.

<h3 markdown>Args<span class="API-type"> : {[string] : string} </span></h3>

Command line arguments passed into the program.


## functions

<h3 markdown>ProcessorCount<span class="API-type"> -> number </span></h3>
```luau
system.ProcessorCount() : number
```
The number of logical CPUs available on the system.

<h3 markdown>PhysicalMemoryUsage<span class="API-type"> -> number </span></h3>
```luau
system.PhysicalMemoryUsage() : number
```
The ammount of physical memory being used up by the process.

<h3 markdown>VirtualMemoryUsage<span class="API-type"> -> number </span></h3>
```luau
system.VirtualMemoryUsage() : number
```
The ammount of virtual memory being used up by the process.

## types

<h3 markdown>LanguagePreferences</h3>

```luau
type LanguagePreferences : {
    time : {string},
    monetary : {string},
    collation : {string},
    message : {string},
    numeric : {string},
}
```