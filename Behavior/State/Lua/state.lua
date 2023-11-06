-- State 接口定义
State = {}

function State:new()
  local newObj = {}
  self.__index = self
  return setmetatable(newObj, self)
end

function State:handle(context)
end

-- 具体状态：StartState
StartState = State:new()

function StartState:handle(context)
  print("Player is in start state")
  context:setState(StopState:new())
end

-- 具体状态：StopState
StopState = State:new()

function StopState:handle(context)
  print("Player is in stop state")
end

-- Context 类，持有状态
Context = {}

function Context:new()
  local newObj = {state = nil}
  self.__index = self
  return setmetatable(newObj, self)
end

function Context:setState(state)
  self.state = state
end

function Context:request()
  self.state:handle(self)
end


-- main.lua

-- 创建一个 Context 对象并给它设置一个初始状态
local context = Context:new()
context:setState(StartState:new())

-- 应用初始状态的行为
context:request()

-- 状态从 StartState 变为 StopState
context:request()

-- 再次调用，但状态是 StopState，应该没有变化
context:request()
