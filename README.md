# vara

Vara is a rust powered luau runtime designed to create standalone applications.
Vara is designed so provide a higher level abstraction for app development without having the bloat of javascript runtimes.

# goals

- crossplatform UI support
- inbuilt media file loading, exporting and usage
- minimal resource usage footprint

# non goals
- manipulating roblox model/place files (use lune) 

# todo

- [ ] CLI
    - [x] run
    - [ ] help page

- [ ] require handling
    - [ ] proper relative paths
    - [ ] .luarc aliases
    
- [ ] standard library
    - [x] io
    - [x] file system
    - [ ] serde