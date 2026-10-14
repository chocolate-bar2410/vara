

--[=[
    @class threadlib

    Inbuilt class for handling concurrency.
]=]
local module = {}

--[=[
    @within threadlib
    @method Sleep
    @param duration
    
    yields for the ammount of time provided in seconds.
]=]
module.Sleep = function(duration : number)
    return nil :: unknown
end

return module