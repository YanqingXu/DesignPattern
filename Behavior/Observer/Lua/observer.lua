-- 观察者类，玩家对象
Player = {}
Player.__index = Player

function Player:new(name)
    local instance = {
        name = name
    }
    setmetatable(instance, Player)
    return instance
end

function Player:update(state)
    print(self.name .. " received update: " .. state)
end

-- 被观察的主题，游戏状态
GameState = {}
GameState.__index = GameState

function GameState:new()
    local instance = {
        observers = {},
        state = ""
    }
    setmetatable(instance, GameState)
    return instance
end

function GameState:attach(observer)
    table.insert(self.observers, observer)
end

function GameState:detach(observer)
    for i, obs in ipairs(self.observers) do
        if obs == observer then
            table.remove(self.observers, i)
        end
    end
end

function GameState:notify()
    for _, observer in ipairs(self.observers) do
        observer:update(self.state)
    end
end

function GameState:setState(state)
    self.state = state
    self:notify()
end

-- 创建游戏状态对象
local game_state = GameState:new()

-- 创建玩家并注册为观察者
local player1 = Player:new("Player1")
local player2 = Player:new("Player2")
local player3 = Player:new("Player3")

game_state:attach(player1)
game_state:attach(player2)
game_state:attach(player3)

-- 更新游戏状态，通知所有玩家
game_state:setState("Level started")
-- 输出:
-- Player1 received update: Level started
-- Player2 received update: Level started
-- Player3 received update: Level started

-- 如果某个玩家离开游戏，从观察者列表中移除
game_state:detach(player2)

-- 再次更新游戏状态
game_state:setState("New item available")
-- 输出:
-- Player1 received update: New item available
-- Player3 received update: New item available
