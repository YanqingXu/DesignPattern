OldCharacter = {}
OldCharacter.__index = OldCharacter

function OldCharacter.new(name)
    local self = setmetatable({}, OldCharacter)
    self.name = name
    return self
end

function OldCharacter:showInfo()
    print("Old system: " .. self.name)
end

NewCharacter = {}
NewCharacter.__index = NewCharacter

function NewCharacter:display()
    error("Abstract method 'display' not implemented")
end

function NewCharacter:playSound()
    error("Abstract method 'playSound' not implemented")
end

CharacterAdapter = {}
CharacterAdapter.__index = CharacterAdapter
setmetatable(CharacterAdapter, { __index = NewCharacter })

function CharacterAdapter.new(oldCharacter)
    local self = setmetatable({}, CharacterAdapter)
    self.oldCharacter = oldCharacter
    return self
end

function CharacterAdapter:display()
    self.oldCharacter:showInfo()
end

function CharacterAdapter:playSound()
    print("Playing sound for: " .. self.oldCharacter.name)
end

-- 创建一个旧的角色
local oldChar = OldCharacter.new("Warrior")

-- 使用适配器将旧的角色适配到新的系统
local adaptedChar = CharacterAdapter.new(oldChar)

-- 在新系统中使用适配后的角色
adaptedChar:display()
adaptedChar:playSound()
