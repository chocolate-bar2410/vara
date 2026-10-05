--!strict
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

local A_decoded = {A = 123}
local A_encoded = 'A = 123\n'

assert(serde.Encode("TOML", A_decoded) == A_encoded,"Failed to encode table")
assert(table_eq(serde.Decode("TOML", A_encoded) :: {string},A_decoded) == true,"Failed to decode table")

local B_decoded = {A = 123,B = {1,2,3}}
local B_encoded = 'A = 123\nB = [1, 2, 3]\n'

assert(serde.Encode("TOML", B_decoded) == B_encoded,"Failed to mixed table")
assert(table_eq(serde.Decode("TOML", B_encoded) :: {string},B_decoded) == true,"Failed to decode mixed table")

local C_decoded = {}
local C_encoded = ''

assert(serde.Encode("TOML", C_decoded) == C_encoded,"Failed to encode empty table")
assert(table_eq(serde.Decode("TOML", C_encoded) :: {string},C_decoded) == true,"Failed to decode empty table")