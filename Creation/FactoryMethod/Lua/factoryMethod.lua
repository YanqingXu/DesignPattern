local IProduct = {}
IProduct.__index = IProduct

function IProduct:new()
    local self = setmetatable({}, IProduct)
    return self
end

function IProduct:show()
    print("IProduct:show()")
end

local ProductA = {}
ProductA.__index = ProductA
setmetatable(ProductA, IProduct)

function ProductA:new()
    local self = setmetatable({}, ProductA)
    return self
end

function ProductA:show()
    print("ProductA:show()")
end

local ProductB = {}
ProductB.__index = ProductB

function ProductB:new()
    local self = setmetatable({}, ProductB)
    return self
end

function ProductB:show()
    print("ProductB:show()")
end

local IFactory = {}
IFactory.__index = IFactory

function IFactory:new()
    local self = setmetatable({}, IFactory)
    return self
end

function IFactory:createProduct()
    print("IFactory:createProduct()")
end

local FactoryA = {}
FactoryA.__index = FactoryA
setmetatable(FactoryA, IFactory)

function FactoryA:new()
    local self = setmetatable({}, FactoryA)
    return self
end

function FactoryA:createProduct()
    print("FactoryA:createProduct()")
    return ProductA:new()
end

local FactoryB = {}
FactoryB.__index = FactoryB
setmetatable(FactoryB, IFactory)

function FactoryB:new()
    local self = setmetatable({}, FactoryB)
    return self
end

function FactoryB:createProduct()
    print("FactoryB:createProduct()")
    return ProductB:new()
end

local factoryA = FactoryA:new()
local productA = factoryA:createProduct()
productA:show()

local factoryB = FactoryB:new()
local productB = factoryB:createProduct()
productB:show()
