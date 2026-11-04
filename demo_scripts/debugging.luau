--!strict

--// normal print

print("hello","world")

--// table print
local tab1 = {
    foo = 123,
    bar = 456,
}

print(tab1)

--// buffer printing

local buffer1 = buffer.fromstring("hello world")
print(buffer1)

--// warn

warn("hello","world")