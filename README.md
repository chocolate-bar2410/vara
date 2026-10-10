# vara

Vara is a rust powered luau runtime designed to create standalone applications.

Vara is designed to provide a higher level abstraction for app development 
without having the bloat associated with javascript runtimes.

vara primarily targets desktop applications with potential mobile support planned for the future.

## goals
What vara aims to achieve.

- inbuilt media file loading, exporting and usage
- minimal resource usage footprint

## future goals
Features planned for a more developed vara

- mobile support.
- crossplatform UI support

## non goals
Intentionally out of scope for vara

- manipulating roblox model/place files (use lune) 
- web development (maybe in the future)

## todo

### v0.1.0

- [x] CLI
    - [x] run
    - [x] build
- [x] standard library
    - [x] io
    - [x] file system
        - [x] directories
    - [x] serde
    - [x] chrono
    - [x] system
    - [x] media
        - [x] images
        - [x] audio
- [x] require
    - [x] relative requires
    - [x] aliases

### v0.2.0
- [ ] CLI
    - [ ] test
    - [ ] build
        - [ ] dependency bundling
- [ ] standard library
    - [ ] networking
    - [ ] task
    - [ ] UI
    - [ ] path
    - [ ] system
        - [ ] notify
- [ ] debugging
    - [ ] custom print
        - [ ] tables
        - [ ] buffers
    - [ ] custom errors
    - [ ] custom warns
    - [ ] test