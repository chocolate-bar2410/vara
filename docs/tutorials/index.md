---
title: Welcome to Vara
---

Vara is a specialised Luau runtime for building standalone applications.

## What is Vara

At its core, Vara aims to provide everything needed to develop and distribute standalone software using Luau.

Vara provides a collection of built in APIs that allow developers to interact with the operating system, manage file systems, process media, manipulate the terminal and so much more.

Whether you are building command line tools, terminal interfaces, media processing pipelines or desktop applications. Vara provides the runtime capabilities and developer tooling to build these projects.

Alongside its runtime APIs, Vara also includes its own script runner, test runner and compilation workflow.  
This gives developers a complete workflow for writing, testing and distributing their applications.

## Why another runtime?

Luau as a language is designed to be embedded into other projects.
Originating as the scripting language for Roblox, Luau has since found applications beyond Roblox, being used by commercial game studios such as Remedy Entertainment and being embedded into platforms like Rive and Gideros.

However much like JavaScript, Luau is designed to be embedded and does not come with host capabilities by default.  
General purpose file system access, serialisation or networking APIs must therefore be provided by the embedding environment.

Runtimes solve this by implementing additional APIs and making them available to the Luau programs they execute.

Existing runtimes like Lune, Lute and [Zune](https://zune.sh/) exist to fill that purpose. These runtimes are general purpose that aim to provide a broad standard library to cater to many different usecases.

While this flexibility is valued and allows all sorts of developers to adopt these runtimes, it can make it difficult to develop specialised software without relying on native extensions or FFI bindings. 
If a developer wanted to process media files or create TUI frameworks. They'd either have to use a runtime with FFI support (like zune or [seal](https://github.com/seal-runtime/seal)) or hope the runtime eventually implements the functionality they need.

This is the gap Vara aims to fill.

## Design goals

* **Luau first** - Developers should be able to build applications entirely in Luau using what's provided by Vara. Developers shouldn't have to write bindings to C++ or Rust programs to use features that should already be in Vara.

* **device first** - Building applications is the primary goal, Vara prioritises access to OS system functionality, proper terminal manipulation and processing native resources over such things as web servers.

!!! info
    Vara is still v0.x software. More features planned and remain in active development.
    All features described in the documentation are fully implemented and ready for use today.

## Inspirations

- **[Astra](https://astra.arkforge.net/)** - A dedicated Lua/Luau runtime for making web applications.

Astra being focused mostly on web development inspired me into making Vara.
Astra demonstrated how a runtime can be specialised around a specific domain. Vara adopts a similar philosophy but for standalone applications instead of web development.

- **[Eryx](https://eryx.bsnk.me/)** - The Luau runtime that provides the entire kitchen sink.

Vara shares a lot of goals with Eryx. However, where Eryx aims to be broadly used across many domains, Vara focuses on having a more cohesive and interconnected API.

- **.NET** - Microsoft's application development platform and runtime.

.NET is one of the most established runtimes that still exists today. With .NET being known for creating desktop apps.
Vara draws inspiration from .NET's philosophy of providing a comprehensive application platform, rather than expecting developers to assemble functionality
from numerous dependencies and bindings. Common capabilities should be directly available through the runtime.