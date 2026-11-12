local path = require("@vara/path")
local system = require("@vara/system")
local unit = require("@vara/test/unit")

local module = {}

function module.Join()
    
    unit.AssertEquals("testing\\hi_guys",path.Join("testing","hi_guys"))
    unit.AssertEquals("testing\\hi_guys\\hello",path.Join("testing","hi_guys","hello"))
    
    unit.AssertEquals("hi_guys",path.Join("","hi_guys"))
    unit.AssertEquals("hi_guys",path.Join("hi_guys",""))

    unit.AssertEquals("",path.Join("",""))

    unit.AssertThrows(path.Join,123,nil)
end

function module.JoinExtension()
    unit.AssertEquals("testing.png",path.WithExt("testing","png"))
    unit.AssertEquals("testing.tar.gz",path.WithExt("testing.tar","gz"))

    unit.AssertEquals("testing",path.WithExt("testing.tar",""))
    unit.AssertEquals("",path.WithExt("","png"))

    unit.AssertThrows(path.WithExt,123,nil)
end

function module.Basename()
    unit.AssertEquals("testing.png",path.Basename("./testing.png"))
    unit.AssertEquals("testing.tar.gz",path.Basename("~/testing.tar.gz"))

    unit.AssertEquals("",path.Basename(""))
end

function module.Absolute()
    unit.AssertEquals(path.Join(system.Cwd,"testing.txt"), path.Absolute("./testing.txt"))
    unit.AssertEquals(path.Join(system.Cwd,"testing.txt"), path.Absolute("././testing.txt"))
end

return module