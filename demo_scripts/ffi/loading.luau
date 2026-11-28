local ffi = require("@vara/ffi")

local lib = ffi.DlOpen(`demo_scripts/ffi/test.{ffi.Suffix}`)
    :Define("add",{"int","int"},"int")

local result = lib.add(2,2)

print(result)