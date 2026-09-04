local contents = buffer.fromstring([[return function()
    print("testing 123")
end]])

fs.write("./demo_scripts/demo2.luau", contents)
