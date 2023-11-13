-- 角色类
Character = {}
Character.__index = Character

function Character.new(name)
    local self = setmetatable({}, Character)
    self.name = name
    self.behavior = nil
    return self
end

function Character:setBehavior(behavior)
    self.behavior = behavior
end

function Character:performAction()
    if self.behavior then
        self.behavior:action()
    else
        print(self.name .. " doesn't know what to do...")
    end
end

-- 行为策略接口
Behavior = {}
Behavior.__index = Behavior

function Behavior.new()
    local self = setmetatable({}, Behavior)
    return self
end

function Behavior:action()
    -- 默认行为或错误提示
    print("Undefined behavior.")
end

-- 攻击行为
AttackBehavior = setmetatable({}, Behavior)
AttackBehavior.__index = AttackBehavior

function AttackBehavior.new()
    return setmetatable({}, AttackBehavior)
end

function AttackBehavior:action()
    print("Attacking the enemy!")
end

-- 防御行为
DefenseBehavior = setmetatable({}, Behavior)
DefenseBehavior.__index = DefenseBehavior

function DefenseBehavior.new()
    return setmetatable({}, DefenseBehavior)
end

function DefenseBehavior:action()
    print("Defending against attacks.")
end

-- 创建一个角色
local hero = Character.new("Hero")

-- 设置角色的行为为攻击
hero:setBehavior(AttackBehavior.new())
hero:performAction()  -- 输出: Attacking the enemy!

-- 改变角色的行为为防御
hero:setBehavior(DefenseBehavior.new())
hero:performAction()  -- 输出: Defending against attacks.
