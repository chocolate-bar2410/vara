local serde = require("@vara/serde")
local unit = require("@vara/test/unit")

local module = {}

--// JSON

function module.JsonArray()
    local decoded = {"A","B","C"}
    local encoded = '["A","B","C"]'

    unit.AssertEquals(serde.Encode("JSON", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("JSON", encoded),decoded)
end

function module.JsonObject()
    local A_decoded = {A = 123,B = 456}
    local A_encoded = '{"A":123,"B":456}'

    unit.AssertEquals(serde.Encode("JSON", A_decoded),A_encoded)
    unit.AssertTableEquals(serde.Decode("JSON", A_encoded), A_decoded)

    local B_decoded = {}
    local B_encoded = "{}"

    unit.AssertEquals(serde.Encode("JSON", B_decoded), B_encoded)
    unit.AssertTableEquals(serde.Decode("JSON", B_encoded), B_decoded)

    local C_decoded = {A = 123,B = {1,2,3}}
    local C_encoded = '{"A":123,"B":[1,2,3]}'

    unit.AssertEquals(serde.Encode("JSON", C_decoded), C_encoded)
    unit.AssertTableEquals(serde.Decode("JSON", C_encoded), C_decoded)
end

--// TOML
function module.TomlObject()
    local decoded = {A = 123}
    local encoded = 'A = 123\n'
    
    unit.AssertEquals(serde.Encode("TOML", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("TOML", encoded), decoded)
end

function module.TomlMixed()
    local decoded = {A = 123,B = {1,2,3}}
    local encoded = 'A = 123\nB = [1, 2, 3]\n'
    
    unit.AssertEquals(serde.Encode("TOML", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("TOML", encoded), decoded)
end

function module.TomlEmpty()
    local decoded = {}
    local encoded = ''
    
    unit.AssertEquals(serde.Encode("TOML", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("TOML", encoded), decoded)
end

--// YAML
function module.YamlObject()
    local decoded = {A = 123}
    local encoded = 'A: 123\n'

    unit.AssertEquals(serde.Encode("YAML", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("YAML", encoded), decoded)
end
function module.YamlMixed()
    local decoded = {A = 123,B = {1,2,3}}
    local encoded = [[A: 123
B:
- 1
- 2
- 3
]]

    unit.AssertEquals(serde.Encode("YAML", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("YAML", encoded), decoded)
end
function module.YamlEmpty()
    local decoded = {}
    local encoded = "{}\n"

    unit.AssertEquals(serde.Encode("YAML", decoded), encoded)
    unit.AssertTableEquals(serde.Decode("YAML", encoded), decoded)
end


function module.EncodeParams()
    unit.AssertDoesNotThrow(serde.Encode,"json",{A = 123})
    unit.AssertDoesNotThrow(serde.Encode,"toml",{A = 123})
    unit.AssertDoesNotThrow(serde.Encode,"yaml",{A = 123})
    unit.AssertThrows(serde.Encode,"idk lmao",{A = 123})
end

function module.DecodeParams()
    unit.AssertDoesNotThrow(serde.Decode,"json",'{"A": 123}')
    unit.AssertDoesNotThrow(serde.Decode,"toml","A = 123")
    unit.AssertDoesNotThrow(serde.Decode,"yaml","A: 123\n")
    unit.AssertThrows(serde.Decode,"idk lmao","{A = 123}")
end

return module

