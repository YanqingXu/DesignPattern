-- 定义事件处理器基类
local EventHandler = {}
function EventHandler:new(o)
    o = o or {}
    setmetatable(o, self)
    self.__index = self
    return o
end

function EventHandler:handleEvent(event)
    -- 基类默认不处理事件，交给下一个处理器处理
    if self.nextHandler then
        self.nextHandler:handleEvent(event)
    end
end

-- 具体的事件处理器类
local ConcreteHandlerA = EventHandler:new()

function ConcreteHandlerA:handleEvent(event)
    if event.type == "A" then
        print("ConcreteHandlerA处理事件A")
    else
        -- 如果不是A类型的事件，交给下一个处理器处理
        EventHandler.handleEvent(self, event)
    end
end

local ConcreteHandlerB = EventHandler:new()

function ConcreteHandlerB:handleEvent(event)
    if event.type == "B" then
        print("ConcreteHandlerB处理事件B")
    else
        -- 如果不是B类型的事件，交给下一个处理器处理
        EventHandler.handleEvent(self, event)
    end
end

-- 创建责任链
local handlerA = ConcreteHandlerA:new()
local handlerB = ConcreteHandlerB:new()
handlerA.nextHandler = handlerB

-- 模拟事件
local event1 = { type = "A" }
local event2 = { type = "B" }
local event3 = { type = "C" }

-- 事件处理
handlerA:handleEvent(event1) -- 输出 "ConcreteHandlerA处理事件A"
handlerA:handleEvent(event2) -- 输出 "ConcreteHandlerB处理事件B"
handlerA:handleEvent(event3) -- 无输出，因为没有处理器处理事件C
