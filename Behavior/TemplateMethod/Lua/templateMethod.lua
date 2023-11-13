-- 基础UI界面类
UIBase = {}
UIBase.__index = UIBase

function UIBase:new()
    local instance = setmetatable({}, self)
    return instance
end

-- 模板方法
function UIBase:build()
    self:initialize()
    self:createElements()
    self:finalize()
end

-- 初始化方法，可被重写
function UIBase:initialize()
    print("Base UI Initialization")
end

-- 创建UI元素，需在子类中重写
function UIBase:createElements()
    error("createElements method should be overridden")
end

-- 最终化方法，可被重写
function UIBase:finalize()
    print("Base UI Finalization")
end

-- 具体的UI界面：MainMenu
MainMenu = UIBase:new()
MainMenu.__index = MainMenu


function MainMenu:initialize()
    print("MainMenu Initialization")
end

function MainMenu:createElements()
    print("Creating MainMenu Elements")
end

function MainMenu:finalize()
    print("MainMenu Finalization")
end

-- 具体的UI界面：SettingsMenu
SettingsMenu = UIBase:new()
SettingsMenu.__index = SettingsMenu

function SettingsMenu:initialize()
    print("SettingsMenu Initialization")
end

function SettingsMenu:createElements()
    print("Creating SettingsMenu Elements")
end

function SettingsMenu:finalize()
    print("SettingsMenu Finalization")
end

function main()
    local mainMenu = MainMenu:new()
    mainMenu:build()

    local settingsMenu = SettingsMenu:new()
    settingsMenu:build()
end

main()
