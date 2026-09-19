# vara

Vara is a rust powered luau runtime designed to create standalone applications.
Vara is designed so provide a higher level abstraction for app development without having the bloat of javascript runtimes.

Applications target desktop and potentially mobile.

# goals

- inbuilt media file loading, exporting and usage
- minimal resource usage footprint

# far goals (stuff i plan on adding at a more stable version)
- crossplatform UI support

# non goals
- manipulating roblox model/place files (use lune) 
- web development (maybe in the future)

# todo

- [x] CLI
    - [x] run
- [ ] standard library
    - [x] io
    - [x] file system
    - [x] serde
    - [x] chrono
    - [ ] process
    - [ ] media
- [x] require
    - [x] relative requires
    - [x] aliases
    