
--[=[
    @class net
    Built in library for networking.
]=]
local module = {}

--[=[
    @within net
    @method HttpClient
    @tag must_use
    @return HttpClient

    Creates a HTTP client.
]=]
module.HttpClient = function() : HttpClient
    return nil :: (unknown)
end

--[=[
    @within net
    @method HttpRequest
    @param url The url of the request.
    @tag must_use
    @return HttpRequest

    Creates a HTTP client.
]=]
module.HttpRequest = function(url: string) : HttpRequest
    return nil :: (unknown)
end

return module