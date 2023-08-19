#include <iostream>
#include <memory>

// 抽象产品类
class Product
{
public:
    virtual void show() = 0;
};

// 具体产品类A
class ConcreteProductA : public Product
{
public:
    void show() override
    {
        std::cout << "Product A" << std::endl;
    }
};

// 具体产品类B
class ConcreteProductB : public Product
{
public:
    void show() override
    {
        std::cout << "Product B" << std::endl;
    }
};

// 抽象工厂类
class Factory
{
public:
    virtual std::unique_ptr<Product> createProduct() = 0;
};

// 具体工厂类A
class ConcreteFactoryA : public Factory
{
public:
    std::unique_ptr<Product> createProduct() override
    {
        return std::make_unique<ConcreteProductA>();
    }
};

// 具体工厂类B
class ConcreteFactoryB : public Factory
{
public:
    std::unique_ptr<Product> createProduct() override
    {
        return std::make_unique<ConcreteProductB>();
    }
};

int main()
{
    // 创建具体工厂A
    std::unique_ptr<Factory> factoryA = std::make_unique<ConcreteFactoryA>();
    // 使用工厂A创建产品A
    std::unique_ptr<Product> productA = factoryA->createProduct();
    productA->show();

    // 创建具体工厂B
    std::unique_ptr<Factory> factoryB = std::make_unique<ConcreteFactoryB>();
    // 使用工厂B创建产品B
    std::unique_ptr<Product> productB = factoryB->createProduct();
    productB->show();

    return 0;
}

/*
在这个示例中：

Product 是抽象产品类，定义了一个 show 方法。
ConcreteProductA 和 ConcreteProductB 是具体的产品类，它们实现了 Product 的 show 方法。

Factory 是抽象工厂类，定义了一个 createProduct 方法。
ConcreteFactoryA 和 ConcreteFactoryB 是具体的工厂类，它们实现了 Factory 的 createProduct 方法，
分别返回 ConcreteProductA 和 ConcreteProductB 的实例。

在 main 函数中，我们创建了两个具体工厂的实例，并使用这些工厂创建了两个具体产品的实例，然后调用了它们的 show 方法。
*/
