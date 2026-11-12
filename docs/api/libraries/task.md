---
icon: octicons/flowchart-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-flowchart-24:</span> 
task
</h1>
Vara's library for managing coroutines.  
`@vara/task` borrows lune's scheduler via [mlua-luau-scheduler](https://crates.io/crates/mlua-luau-scheduler), which allows vara to achieve almost full compatibility with roblox's task library.

---

## functions

<h3 markdown>Sleep<span class="API-type"> -> () </span></h3>
```luau
task.Sleep(duration : number?)
```
Yields for the ammount of time provided in seconds.  
If no duration was given it will default to the minimum of 1ms.

<h3 markdown>Spawn<span class="API-type"> -> thread </span></h3>
```luau
task.Spawn(
    callback: () -> () | thread,
    ...
): thread
```
Creates and runs a new thread or function 

<h3 markdown>Defer<span class="API-type"> -> thread </span></h3>
```luau
task.Defer(
    callback: () -> () | thread,
    ...
): thread
```
Like task.Spawn but defers the thread to the end of the current task queue. 

<h3 markdown>Cancel<span class="API-type"> -> () </span></h3>
```luau
task.Cancel(thread: thread)
```
Stops the thread provided thats currently scheduled from resuming.

<h3 markdown>All<span class="API-type"> -> () </span></h3>
```luau
task.All(...: () -> () | thread)
```
Concurrently runs each provided thread until they all finished.  
Functions that are passed in get converted to threads.

<h3 markdown>Race<span class="API-type"> -> () </span></h3>
```luau
task.Race(...: () -> () | thread): (...unknown)
```
Concurrently runs each provided thread until atleast one of them is finished.  
Functions that are passed in get converted to threads.

The result is the return value of the winning task.