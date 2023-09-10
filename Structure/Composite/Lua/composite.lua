-- 抽象基类表示游戏角色和装备
local GameObject = {}

function GameObject:new(name)
    local newObj = { name = name, components = {} }
    self.__index = self
    return setmetatable(newObj, self)
end

function GameObject:addComponent(component)
    table.insert(self.components, component)
end

function GameObject:removeComponent(component)
    for i, comp in ipairs(self.components) do
        if comp == component then
            table.remove(self.components, i)
            return
        end
    end
end

function GameObject:display()
    print("Name: " .. self.name)
    for _, component in ipairs(self.components) do
        component:display()
    end
end

-- 叶子节点：角色
local Character = {}
setmetatable(Character, { __index = GameObject })

-- 叶子节点：装备
local Equipment = {}
setmetatable(Equipment, { __index = GameObject })

-- 组合节点：装备组合
local EquipmentComposite = {}
setmetatable(EquipmentComposite, { __index = GameObject })

function EquipmentComposite:new(name)
    local newObj = GameObject:new(name)
    self.__index = self
    return setmetatable(newObj, self)
end

function EquipmentComposite:display()
    print("Equipment Group: " .. self.name)
    for _, component in ipairs(self.components) do
        component:display()
    end
end

-- 创建游戏角色和装备
local character = Character:new("Player")
local sword = Equipment:new("Sword")
local shield = Equipment:new("Shield")

-- 创建装备组合
local equipmentGroup = EquipmentComposite:new("Armor Set")
equipmentGroup:addComponent(sword)
equipmentGroup:addComponent(shield)

-- 将装备组合添加到角色中
character:addComponent(equipmentGroup)

-- 显示角色及其装备
character:display()
