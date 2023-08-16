-- singleton.lua
local Singleton = {}
local instance = nil

function Singleton:new(o, ...)
    if instance == nil then
        o = o or {}
        setmetatable(o, self)
        self.__index = self
        instance = o
    end
    return instance
end

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
