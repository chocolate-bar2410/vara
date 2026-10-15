--[=[
    @class threadlib

    Inbuilt class for handling co operative multitasking.
]=]
local module = {}

--[=[
    @within threadlib
    @method Sleep
    @param duration
    
    yields for the ammount of time provided in seconds.
    If no duration was given it will default to the minimum of 1ms.
]=]
module.Sleep = function(duration : number?)
    return nil :: unknown
end

--[=[
    @within threadlib
    @method Spawn
    @param callback The thread/function used.
    
    Creates and runs a new thread or function 
]=]
module.Spawn = function(callback: () -> () | thread,...): thread
    return nil :: unknown
end

--[=[
    @within threadlib
    @method Defer
    @param callback The thread/function used.
    
    Like task.Spawn but defers the thread to the end of the current task queue. 
]=]
module.Defer = function(callback: () -> () | thread,...): thread
    return nil :: unknown
end

--[=[
    @within threadlib
    @method Cancel
    @param callback The thread being cancelled.
    
    Stops the thread provided thats currently scheduled from resuming.
]=]
module.Cancel = function(thread: thread)
    return nil :: unknown
end

--[=[
    @within threadlib
    @method All
    @param ... tasks to be ran.
    
    Concurrently runs each provided thread until they all finished.
    Functions that are passed in get converted to threads.
]=]
module.All = function(...:( ) -> () | thread)
    return nil :: unknown
end

--[=[
    @within threadlib
    @method Race
    @param ... tasks to be ran.
    
    Concurrently runs each provided thread until atleast one of them is finished.
    Functions that are passed in get converted to threads.
]=]
module.Race = function(...:( ) -> () | thread)
    return nil :: unknown
end

return module