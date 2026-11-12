--!strict
-- for testing requires and compilation
local config = require("@self/configs")
local _unused_variable

print(config.hello)
-- random comment
print(config.world)

if 2 < 5 then 
    print("hi guys")
else
    print("unused if")
end

while false do
    print("unused whileloop")
end

const PHI = (math.sqrt(5) + 1) / 2

print(PHI)