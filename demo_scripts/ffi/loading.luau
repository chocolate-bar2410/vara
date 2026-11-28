local ffi = require("@vara/ffi")

local lib = ffi.DlOpen("demo_scripts/ffi/test.dll")
    :Define("add",{"int","int"},"int")

    