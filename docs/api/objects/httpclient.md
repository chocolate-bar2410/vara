---
icon: octicons/server-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-server-24:</span> 
HttpClient
</h1>

A object that sends http requests.

---

## properties

<h3 markdown>Timeout<span class="API-type"> : number</span></h3>
How long the http request lasts until it cancels.  
Defaults to 30 seconds.

<h3 markdown>Headers<span class="API-type"> : {[string]: string}</span></h3>
The header information provided to http requests.

---

## methods

<h3 markdown>SendAsync<span class="API-type"> -> HttpResponse </span></h3>
```luau
HttpClient:SendAsync(request: HttpRequest) : HttpResponse
```
Sends a http request to the given url.

<h3 markdown>AddHeader<span class="API-type"> -> HttpClient </span></h3>
```luau
HttpClient:AddHeader(
    key: string,
    value: string
) : HttpClient
```
Adds a header to the http client.  
Returns itself.

<h3 markdown>SetTimeout<span class="API-type"> -> HttpClient </span></h3>
```luau
HttpClient:SetTimeout(duration: number) : HttpClient
```
Helper method for setting the timeout.  
Returns itself.