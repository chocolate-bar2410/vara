local serde = require("@vara/serde")

local function table_eq<T>(A: {T},B: {T})
    if #A ~= #B then return false end

    for i in A do

        if typeof(A[i]) ~= typeof(B[i]) then return false end

        if typeof(A[i]) == "table" then return table_eq(A[i],B[i]) end

       if A[i] ~= B[i] then return false end 
    end

    return true
end

local A_decoded = {"A","B","C"}
local A_encoded = '["A","B","C"]'

assert(serde.encode("JSON", A_decoded) == A_encoded,"Failed to encode JSON array")
assert(table_eq(serde.decode("JSON", A_encoded) :: {string},A_decoded) == true,"Failed to decode JSON array")

local B_decoded = {A = 123,B = 456}
local B_encoded = '{"A":123,"B":456}'

assert(serde.encode("JSON", B_decoded) == B_encoded,"Failed to encode table")
assert(table_eq(serde.decode("JSON", B_encoded) :: {string},B_decoded) == true,"Failed to decode JSON object")

local C_decoded = {}
local C_encoded = "{}"

assert(serde.encode("JSON", C_decoded) == C_encoded,"Failed to encode empty table")
assert(table_eq(serde.decode("JSON", C_encoded) :: {string},C_decoded) == true,"Failed to encode empty JSON object")

local D_decoded = {A = 123,B = {1,2,3}}
local D_encoded = '{"A":123,"B":[1,2,3]}'

assert(serde.encode("JSON", D_decoded) == D_encoded,"Failed to encode mixed table")
assert(table_eq(serde.decode("JSON", D_encoded) :: {string},D_decoded) == true,"Failed to encode mixed object")

assert(pcall(serde.encode,"json",{A = 123}) == true)
assert(pcall(serde.encode,"toml",{A = 123}) == true)
assert(pcall(serde.encode,"yaml",{A = 123}) == true)
assert(pcall(serde.encode,"idk lmao",{A = 123}) == false)

assert(pcall(serde.decode,"json",'{"A": 123}') == true)
assert(pcall(serde.decode,"toml","A = 123") == true)
assert(pcall(serde.decode,"yaml","A: 123\n") == true)
assert(pcall(serde.decode,"idk lmao","{A = 123}") == false)