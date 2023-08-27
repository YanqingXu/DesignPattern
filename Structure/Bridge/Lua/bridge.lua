-- 抽象类：ResourceManager
ResourceManager = {}
ResourceManager.__index = ResourceManager

function ResourceManager:new(loader)
    local self = setmetatable({}, ResourceManager)
    self.loader = loader
    return self
end

function ResourceManager:loadResource(resourceName)
    self.loader:load(resourceName)
end

-- 具体实现接口：ResourceLoader
ResourceLoader = {}
ResourceLoader.__index = ResourceLoader

function ResourceLoader:load(resourceName)
end

-- 具体实现类：ImageLoader
ImageLoader = {}
ImageLoader.__index = ImageLoader

function ImageLoader:load(resourceName)
    print("Loading image resource: " .. resourceName)
end

-- 具体实现类：AudioLoader
AudioLoader = {}
AudioLoader.__index = AudioLoader

function AudioLoader:load(resourceName)
    print("Loading audio resource: " .. resourceName)
end

-- 具体实现类：TextLoader
TextLoader = {}
TextLoader.__index = TextLoader

function TextLoader:load(resourceName)
    print("Loading text resource: " .. resourceName)
end

-- 测试代码
local imageManager = ResourceManager:new(ImageLoader)
local audioManager = ResourceManager:new(AudioLoader)
local textManager = ResourceManager:new(TextLoader)

imageManager:loadResource("background.png")
audioManager:loadResource("background_music.mp3")
textManager:loadResource("story.txt")
