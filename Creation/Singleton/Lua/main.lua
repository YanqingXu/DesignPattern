local Singleton = require("singleton")

local MyClass = Singleton:new()

function MyClass:display()
    print("MyClass instance!")
end

local instance1 = MyClass:new()
local instance2 = MyClass:new()

instance1:display()
instance2:display()

if instance1 == instance2 then
    print("两个实例是相同的")
else
    print("两个实例是不同的")
end
