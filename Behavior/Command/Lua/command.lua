-- Character 类表示游戏中的角色
Character = {
    position = 0,
}

function Character:move(distance)
    self.position = self.position + distance
    print("Moved", distance, "units. Current position:", self.position)
end

-- MoveCommand 表示移动命令
MoveCommand = {}

function MoveCommand:new(character, distance)
    local command = {}
    setmetatable(command, self)
    self.__index = self
    command.character = character
    command.distance = distance
    return command
end

function MoveCommand:execute()
    self.character:move(self.distance)
end

-- CommandQueue 类用于管理命令队列
CommandQueue = {
    commands = {},
    currentIndex = 0,
}

function CommandQueue:addCommand(command)
    table.insert(self.commands, command)
end

function CommandQueue:getNextCommand()
    self.currentIndex = self.currentIndex + 1
    if self.currentIndex <= #self.commands then
        return self.commands[self.currentIndex]
    else
        return nil
    end
end

-- 网络同步模拟
function simulateNetworkSync()
    local player = Character
    local commandQueue = CommandQueue

    -- 在网络上接收到命令
    local moveCommand1 = MoveCommand:new(player, 5)
    local moveCommand2 = MoveCommand:new(player, -3)

    -- 添加命令到队列
    commandQueue:addCommand(moveCommand1)
    commandQueue:addCommand(moveCommand2)

    -- 执行队列中的命令
    local command = commandQueue:getNextCommand()
    while command do
        command:execute()
        command = commandQueue:getNextCommand()
    end
end

-- 模拟网络同步
simulateNetworkSync()
