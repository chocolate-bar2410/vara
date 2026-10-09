---
hide:
  - navigation
  - toc
---

<div class="homepage-main" markdown>
<div class="homepage-inner" markdown>
# Vara { .homepage-title } 

Vara is a lightweight runtime that provides native system access, media processing and resource management.<br>
Build applications without the overhead of web runtimes.<br><br>
Turn luau into a language for desktop development.

=== "ember"
    ```
    embr tool add chocolate-bar2410/vara
    ```

=== "rokit"
    ```
    rokit add chocolate-bar2410/vara
    ```
</div>
<div class="homepage-code_example" markdown>

=== "mini fast fetch"
    ```luau
    const system = require("@vara/system")
    print(system.Device)
    print("-----------------------------------")
    print(`OS: {system.Os} {system.Distro} {system.Arch}`)
    print(`DE: {system.DesktopEnvironment}`)
    print(`Language: {system.LanguagePreferences.Message[1]}`)
    ```

    ```
    > vara run fast_fetch.luau
    ```
=== "Image pipeline"
    ```luau
    const fs = require("@vara/fs")
    const media = require("@vara/media")
    
    const kernel = {
        1, 2, 1,
        2, 4, 2,
        1, 2, 1
    }

    local image = media.Image("example.png")
        :Resize(512,512,"LANCZOS3")
        :Convolve(3,3,kernel)
        :Flip(0)
        :Flop(1)
        
    fs.File("example.qoi"):Write(image:Encode("QOI"))
    ```
    ```
    > vara run image_pipeline.luau
    ```


</div>

</div>
<br>

---

<br>
<div class="homepage-feature_list">
<span class="homepage-feature" markdown> 
<div class="feature-title">Native media processing</div>
<div class="feature-desc" markdown>
Load, Transform and export media directly from luau.<br/>
Build tooling and automated pipelines for image and audio processing.<br/>
</div>
</span>

<span class="homepage-feature" markdown> 
<div class="feature-title">Operating system access</div>
<div class="feature-desc" markdown>
Access information on the operating system, device details, and user preferences directly from Luau.<br/>
</div>
</span>

<span class="homepage-feature" markdown> 
<div class="feature-title">Native resource objects</div>
<div class="feature-desc" markdown>
Files, directories, and other resources are encapsulated as luau objects with their own properties and methods.
</div>
</span>
</div>
<br>