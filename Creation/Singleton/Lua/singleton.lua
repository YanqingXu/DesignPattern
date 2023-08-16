-- singleton.lua
local Singleton = {}
local instance = nil

function Singleton:new(o, ...)
    if instance == nil then
        o = o or {}
        setmetatable(o, self)
        self.__index = self
        instance = o
    end
    return instance
end

return Singleton
