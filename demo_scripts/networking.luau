--// libraries
local net = require("@vara/net")
local serde = require("@vara/serde")

--// variables

local title_string = "\x1b[33m\u{f005}\x1b[0m %d - \x1b[1m%s\x1b[0m"
local client = net.HttpClient()
    :SetTimeout(5)

local request = net.HttpRequest("https://api.github.com/search/repositories")

--// logic

request:AddQuery("q", "language:luau")
       :AddQuery("sort", "stars")
       :AddQuery("per_page", "3")

local response = client:SendAsync(request)

if response.Ok == false then 
    return 
end

local body = buffer.tostring(response.Body)
local repositories = serde.Decode("JSON",body)

for _,repository in repositories.items do
    print(string.format(
        title_string,
        repository.stargazers_count,
        repository.full_name
    ))
    print(repository.description .. "\n")
end
