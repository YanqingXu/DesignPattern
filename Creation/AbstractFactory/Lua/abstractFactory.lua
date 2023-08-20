-- 环境、敌人、障碍物和奖励的抽象类
Environment = {}
function Environment:describe() end

Enemy = {}
function Enemy:describe() end

Obstacle = {}
function Obstacle:describe() end

Reward = {}
function Reward:describe() end

-- 森林环境
ForestEnvironment = {}
setmetatable(ForestEnvironment, { __index = Environment })
function ForestEnvironment:describe()
    return "Lush forest"
end

-- 沙漠环境
DesertEnvironment = {}
setmetatable(DesertEnvironment, { __index = Environment })
function DesertEnvironment:describe()
    return "Hot desert"
end

-- 狼敌人
Wolf = {}
setmetatable(Wolf, { __index = Enemy })
function Wolf:describe()
    return "A wild wolf"
end

-- 蝎子敌人
Scorpion = {}
setmetatable(Scorpion, { __index = Enemy })
function Scorpion:describe()
    return "A deadly scorpion"
end

-- 树障碍物
TreeObstacle = {}
setmetatable(TreeObstacle, { __index = Obstacle })
function TreeObstacle:describe()
    return "A large tree"
end

-- 沙丘障碍物
SandDune = {}
setmetatable(SandDune, { __index = Obstacle })
function SandDune:describe()
    return "A shifting sand dune"
end

-- 血瓶奖励
HealthPotion = {}
setmetatable(HealthPotion, { __index = Reward })
function HealthPotion:describe()
    return "A health potion"
end

-- 水瓶奖励
WaterBottle = {}
setmetatable(WaterBottle, { __index = Reward })
function WaterBottle:describe()
    return "A bottle of water"
end

-- 关卡抽象工厂
LevelFactory = {}

-- 创建环境
function LevelFactory:createEnvironment() end

-- 创建敌人
function LevelFactory:createEnemy() end

-- 创建障碍物
function LevelFactory:createObstacle() end

-- 创建奖励
function LevelFactory:createReward() end

-- 森林关卡工厂
ForestLevelFactory = {}
setmetatable(ForestLevelFactory, { __index = LevelFactory })
function ForestLevelFactory:createEnvironment()
    return ForestEnvironment
end

function ForestLevelFactory:createEnemy()
    return Wolf
end

function ForestLevelFactory:createObstacle()
    return TreeObstacle
end

function ForestLevelFactory:createReward()
    return HealthPotion
end

DesertLevelFactory = {}
setmetatable(DesertLevelFactory, { __index = LevelFactory })
function DesertLevelFactory:createEnvironment()
    return DesertEnvironment
end

function DesertLevelFactory:createEnemy()
    return Scorpion
end

function DesertLevelFactory:createObstacle()
    return SandDune
end

function DesertLevelFactory:createReward()
    return WaterBottle
end

local forestFactory = ForestLevelFactory
print(forestFactory:createEnvironment():describe())
print(forestFactory:createEnemy():describe())
print(forestFactory:createObstacle():describe())
print(forestFactory:createReward():describe())

local desertFactory = DesertLevelFactory
print(desertFactory:createEnvironment():describe())
print(desertFactory:createEnemy():describe())
print(desertFactory:createObstacle():describe())
print(desertFactory:createReward():describe())
