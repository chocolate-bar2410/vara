--!strict

--// normal print

print("hello","world")

--// table print
local tab1 = {
    foo = 123,
    bar = 456,
}

print(tab1)

local tab2 = {
    foo = 123,
    bar = 456,
    hi = {
        abc = "abc",
        [123] = "hello"
    }
}

print(tab2)

local tab3 = { bar = 456}
local tab4 = { bar = 456,tab = tab3}
local tab5 = { foo = 123,hi = tab4, hello = tab4}

tab3.tab = tab4
print(tab5)

--// buffer printing

local buffer1 = buffer.fromstring("hello world abcdefghijklmnopqrstuvvwxyz")
print(buffer1)

--// warn

warn("hello","world")