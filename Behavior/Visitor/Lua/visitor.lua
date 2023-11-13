-- 元素接口
Element = {}
function Element:new()
    local obj = {}
    setmetatable(obj, self)
    self.__index = self
    return obj
end
function Element:accept(visitor)
    -- 将由具体元素覆盖
end

-- 角色
Character = Element:new()
function Character:accept(visitor)
    visitor:visitCharacter(self)
end

-- 敌人
Enemy = Element:new()
function Enemy:accept(visitor)
    visitor:visitEnemy(self)
end

-- 道具
Item = Element:new()
function Item:accept(visitor)
    visitor:visitItem(self)
end

-- 访问者接口
Visitor = {}
function Visitor:new()
    local obj = {}
    setmetatable(obj, self)
    self.__index = self
    return obj
end
function Visitor:visitCharacter(character)
    -- 将由具体访问者覆盖
end
function Visitor:visitEnemy(enemy)
    -- 将由具体访问者覆盖
end
function Visitor:visitItem(item)
    -- 将由具体访问者覆盖
end

-- 游戏规则访问者
GameRulesVisitor = Visitor:new()
function GameRulesVisitor:visitCharacter(character)
    print("Applying game rules to a character")
    -- 这里添加实际的游戏规则逻辑
end
function GameRulesVisitor:visitEnemy(enemy)
    print("Applying game rules to an enemy")
    -- 这里添加实际的游戏规则逻辑
end
function GameRulesVisitor:visitItem(item)
    print("Applying game rules to an item")
    -- 这里添加实际的游戏规则逻辑
end


function main()
    local elements = {Character:new(), Enemy:new(), Item:new()}

    local gameRulesVisitor = GameRulesVisitor:new()

    for _, element in ipairs(elements) do
        element:accept(gameRulesVisitor)
    end
end

main()
