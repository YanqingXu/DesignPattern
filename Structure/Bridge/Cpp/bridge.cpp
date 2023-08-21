#include <iostream>
#include <memory>

// 实现接口
class Implementor
{
public:
    virtual void operationImpl() = 0;
};

// 具体实现A
class ConcreteImplementorA : public Implementor
{
public:
    void operationImpl() override
    {
        std::cout << "ConcreteImplementorA operationImpl" << std::endl;
    }
};

// 具体实现B
class ConcreteImplementorB : public Implementor
{
public:
    void operationImpl() override
    {
        std::cout << "ConcreteImplementorB operationImpl" << std::endl;
    }
};

// 抽象基类
class Abstraction
{
protected:
    std::unique_ptr<Implementor> implementor;

public:
    Abstraction(std::unique_ptr<Implementor> impl) : implementor(std::move(impl)) {}
    virtual void operation()
    {
        implementor->operationImpl();
    }
};

// 扩展抽象类
class RefinedAbstraction : public Abstraction
{
public:
    RefinedAbstraction(std::unique_ptr<Implementor> impl) : Abstraction(std::move(impl)) {}
    void operation() override
    {
        std::cout << "RefinedAbstraction operation" << std::endl;
        implementor->operationImpl();
    }
};

int main()
{
    auto absA = std::make_unique<RefinedAbstraction>(std::make_unique<ConcreteImplementorA>());
    absA->operation();

    auto absB = std::make_unique<RefinedAbstraction>(std::make_unique<ConcreteImplementorB>());
    absB->operation();

    absA = std::move(absB);
    absA->operation();

    return 0;
}
