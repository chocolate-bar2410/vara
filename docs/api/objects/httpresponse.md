---
icon: octicons/calendar-16
---

<h1 class = "API-header" markdown> 
<span class="API-icon" markdown>:octicons-calendar-24:</span> 
HttpResponse
</h1>

The result of a http request.

---

## properties
<h3 markdown>Status<span class="API-type"> : number</span></h3>
The status code returned.

<h3 markdown>Body<span class="API-type"> : buffer</span></h3>
The contents of the http response.

<h3 markdown>Ok<span class="API-type"> : boolean</span></h3>
Whether the http request was sucessful or not.

<h3 markdown>Headers<span class="API-type"> : {[string]: string}</span></h3>
Header data returned from the http request.

<h3 markdown>Url<span class="API-type"> : string</span></h3>
The effective URI of the http response.  
This value differs from the original URI provided when making the request if at least one redirect was followed.