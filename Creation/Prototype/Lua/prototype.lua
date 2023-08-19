-- 原型接口
Prototype = {}

function Prototype:new(o, data)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    self.data = data
    return o
end

function Prototype:clone()
    return Prototype:new(nil, self.data)
end

function Prototype:operation()
    print("ConcretePrototype operation with data: " .. self.data)
end

-- 使用示例
local prototype = Prototype:new(nil, 10)
local clonedPrototype = prototype:clone()

prototype:operation()
clonedPrototype:operation()
