
--[=[
	@class HttpResponse

    A object that sends http requests.
]=]
declare extern type HttpResponse with
    read Status: number
    read Body: buffer
    read Ok: boolean
    read Headers: {[string]: string}
    read Url: string
end

type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD" | "OPTIONS" | "CONNECT" | "TRACE"


--[=[
	@class HttpRequest

    A object that sends http requests.
]=]
declare extern type HttpRequest with
    Method: HttpMethod
    Query: {[string]: string}
    Body: buffer
    --[=[
        @within HttpRequest
        @method AddQuery
        @param key The key value of the query.
        @param value The value of the query.
        @return HttpResponse
        
        Adds a query to the http request.
    ]=]
    function AddQuery(self,key: string,value: string) : HttpRequest
end

--[=[
	@class HttpClient

    A object that sends http requests.
]=]
declare extern type HttpClient with
    Timeout: number
    Headers: {[string]: string}
    --[=[
        @within HttpClient
        @method SendAsync
        @param url The URL to send the request to.
        @return HttpResponse
        
        Sends a http request to the given url.
    ]=]
    function SendAsync(self,request: HttpRequest) : HttpResponse
    --[=[
        @within HttpClient
        @method AddHeader
        @param key The key value of the header.
        @param value The value of the header.
        @return itself
        
        Adds a header to the http client.
    ]=]
    function AddHeader(self,key: string,value: string) : HttpClient
    --[=[
        @within HttpClient
        @method SetTimeout
        @param duration The duration the
        @return itself
        
        Helper method for setting the timeout
    ]=]
    function SetTimeout(self,duration: number) : HttpClient
end