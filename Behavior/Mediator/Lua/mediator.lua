-- 中介者类
local Mediator = {}
function Mediator:new()
    local obj = { colleagues = {} }
    self.__index = self
    return setmetatable(obj, self)
end

function Mediator:register(colleague)
    table.insert(self.colleagues, colleague)
end

function Mediator:send(message, sender)
    for _, colleague in ipairs(self.colleagues) do
        if colleague ~= sender then
            colleague:receive(message)
        end
    end
end

-- 同事基类
local Colleague = {}
function Colleague:new(mediator)
    local obj = { mediator = mediator }
    self.__index = self
    return setmetatable(obj, self)
end

function Colleague:send(message)
    self.mediator:send(message, self)
end

function Colleague:receive(message)
    -- 由子类实现
end

-- 具体的同事类
local ConcreteColleague1 = Colleague:new()

function ConcreteColleague1:receive(message)
    print("ConcreteColleague1 received: " .. message)
end

local ConcreteColleague2 = Colleague:new()

function ConcreteColleague2:receive(message)
    print("ConcreteColleague2 received: " .. message)
end

-- 使用中介者模式
local mediator = Mediator:new()

local colleague1 = ConcreteColleague1:new(mediator)
local colleague2 = ConcreteColleague2:new(mediator)

mediator:register(colleague1)
mediator:register(colleague2)

colleague1:send("Hello")
colleague2:send("Good day")
