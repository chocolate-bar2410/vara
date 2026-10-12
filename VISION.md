# project vision

## overview
Vara is a rust powered standalone runtime for luau built around app development, primarily targetting desktop apps.
Vara acts as a toolkit providing access to features developers would use when making an application.

## principles
* **luau first** - developers shouldn't have to program stuff in C++/rust to make an app using vara
    - Vara itself should provide everything a developer would need when making an app.
    - If a developer has to rely on external bindings for a feature that vara should have, vara has failed as a runtime.

* **device first** - applications that run on the device should be the primary goal
    - features like server sided applications and webviews should be low priority.

## scope
* **goals** - What vara aims to achieve.
    - inbuilt media file loading, exporting and usage
    - minimal resource usage footprint

* **future goals** - Features planned for a more developed vara
    - mobile SDK and tooling.
    - crossplatform UI support

* **non goals** - Intentionally out of scope for vara
    - manipulating roblox model/place files (use lune) 
    - web development