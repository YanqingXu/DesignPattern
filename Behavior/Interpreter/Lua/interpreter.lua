-- 定义红点类
RedDot = {}
RedDot.__index = RedDot

function RedDot:new()
    local self = setmetatable({}, RedDot)
    self.children = {}  -- 子红点列表
    self.isDirty = true  -- 红点状态
    return self
end

function RedDot:addChild(child)
    table.insert(self.children, child)
end

function RedDot:removeChild(child)
    for i, c in ipairs(self.children) do
        if c == child then
            table.remove(self.children, i)
            break
        end
    end
end

function RedDot:check()
    for _, child in ipairs(self.children) do
        if child.isDirty then
            return true
        end
    end
    return false
end

-- 定义解释器类
Interpreter = {}
Interpreter.__index = Interpreter

function Interpreter:new()
    return setmetatable({}, Interpreter)
end

function Interpreter:interpret(redDot)
    if redDot:check() then
        print("显示红点")
    else
        print("隐藏红点")
    end
end

-- 示例
local rootRedDot = RedDot:new()
local childRedDot1 = RedDot:new()
local childRedDot2 = RedDot:new()

rootRedDot:addChild(childRedDot1)
rootRedDot:addChild(childRedDot2)

local interpreter = Interpreter:new()
interpreter:interpret(rootRedDot)  -- 输出 "显示红点"
