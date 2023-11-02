-- Memento类
local Memento = {}
Memento.__index = Memento

function Memento:new(state)
    local m = setmetatable({}, Memento)
    m.state = state
    return m
end

function Memento:getState()
    return self.state
end

-- Originator类
local Originator = {}
Originator.__index = Originator

function Originator:new()
    return setmetatable({ state = "" }, Originator)
end

function Originator:setState(state)
    self.state = state
end

function Originator:getState()
    return self.state
end

function Originator:saveToMemento()
    return Memento:new(self.state)
end

function Originator:restoreFromMemento(memento)
    self.state = memento:getState()
end

-- Caretaker类
local Caretaker = {}
Caretaker.__index = Caretaker

function Caretaker:new()
    return setmetatable({ mementos = {} }, Caretaker)
end

function Caretaker:addMemento(memento)
    table.insert(self.mementos, memento)
end

function Caretaker:getMemento(index)
    return self.mementos[index]
end

-- 使用备忘录模式的示例
local originator = Originator:new()
local caretaker = Caretaker:new()

-- 设置初始状态
originator:setState("State #1")
-- 保存状态
caretaker:addMemento(originator:saveToMemento())

-- 改变状态
originator:setState("State #2")

-- 从备忘录中恢复状态
originator:restoreFromMemento(caretaker:getMemento(1))
print("Current State: " .. originator:getState())

-- 结果输出将会是 "State #1"
