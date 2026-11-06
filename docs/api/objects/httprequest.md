---
icon: octicons/paper-airplane-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-paper-airplane-24:</span> 
HttpRequest
</h1>

A object that represents a http request.

---

## properties

<h3 markdown>Method<span class="API-type"> : HttpMethod</span></h3>
The HTTP method associated with the request.

<h3 markdown>Query<span class="API-type"> : {[string]: string}</span></h3>
Query data for the request.

<h3 markdown>Body<span class="API-type"> : buffer</span></h3>
The body of the http request.

<h3 markdown>Url<span class="API-type"> : String</span></h3>
The url that the request is being sent to.

---

## methods

<h3 markdown>AddQuery<span class="API-type"> -> HttpRequest </span></h3>
```luau
HttpRequest:AddQuery(key: string,value: string) : HttpRequest
```
Adds a query to the http request.

## types

<h3 markdown>HttpMethod</h3>
```luau
type HttpMethod = "GET" | "POST" | "PUT" | "PATCH" | "DELETE" | "HEAD" | "OPTIONS" | "CONNECT" | "TRACE"
```
