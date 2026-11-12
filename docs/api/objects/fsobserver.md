---
icon: octicons/eye-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-eye-24:</span> 
FsObserver
</h1>

An object that watches files and directories.

---

## properties

<h3 markdown>Path<span class="API-type"> : string</span></h3>
The file/directory being watched.

<h3 markdown>OnCreated<span class="API-type"> : (paths: {string}) -> ()</span></h3>
Callback that fires when a new file is created in the directory being watched.

<h3 markdown>OnModified<span class="API-type"> : (paths: {string}) -> ()</span></h3>
Callback that fires when the file/directory being watched is modified.

<h3 markdown>OnRemoved<span class="API-type"> : (paths: {string}) -> ()</span></h3>
Callback that fires when either the file being watched is removed or if a file inside the directory being watched is removed.

---

## methods

<h3 markdown>Disconnect<span class="API-type"> -> () </span></h3>
```luau
FsObserver:Disconnect()
```
Stops the observer from observing file events.  
