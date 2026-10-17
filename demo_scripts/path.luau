local path = require("@vara/path")

local my_path = path.Join("demo_scripts","images","resizing_demo","cameraman_bilinear")
my_path = path.WithExt(my_path,"png")
print(my_path)