-- 基本游戏对象
local GameObject = {}

function GameObject:new(name)
    local newObj = { name = name }
    self.__index = self
    return setmetatable(newObj, self)
end

function GameObject:render()
    print("Rendering " .. self.name)
end

-- 特效和效果的装饰器
local EffectDecorator = {}

function EffectDecorator:new(obj, effect)
    local newObj = { object = obj, effect = effect }
    self.__index = self
    return setmetatable(newObj, self)
end

function EffectDecorator:render()
    self.object:render()
    print("Applying Effect: " .. self.effect)
end

-- 创建基本游戏对象
local player = GameObject:new("Player")

-- 添加特效和效果
local playerWithFireEffect = EffectDecorator:new(player, "Fire Effect")
local playerWithIceEffect = EffectDecorator:new(player, "Ice Effect")

-- 渲染游戏对象和其特效
player:render()
print()

playerWithFireEffect:render()
print()

playerWithIceEffect:render()
