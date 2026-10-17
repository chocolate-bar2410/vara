local path = require("@vara/path")

local my_path = path.Join("demo_scripts","images","resizing_demo","cameraman_bilinear")
my_path = path.WithExt(my_path,"png")
my_path = path.Absolute(my_path)
print(my_path)