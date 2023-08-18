-- 定义Car类
local Car = {}
Car.__index = Car

function Car.new()
    local self = setmetatable({}, Car)
    self.Wheels = nil
    self.Engine = nil
    self.Color = nil
    return self
end

-- 定义CarBuilder类
local CarBuilder = {}
CarBuilder.__index = CarBuilder

function CarBuilder.new()
    local self = setmetatable({}, CarBuilder)
    self.car = Car.new()
    return self
end

function CarBuilder:setWheels(wheels)
    self.car.Wheels = wheels
    return self
end

function CarBuilder:setEngine(engine)
    self.car.Engine = engine
    return self
end

function CarBuilder:setColor(color)
    self.car.Color = color
    return self
end

function CarBuilder:build()
    return self.car
end

-- 创建Car实例
local car = CarBuilder.new()
    :setWheels(4)
    :setEngine("V8")
    :setColor("Red")
    :build()

print("Car: ")
print("Wheels: " .. car.Wheels)
print("Engine: " .. car.Engine)
print("Color: " .. car.Color)
