#include <iostream>
#include <memory>

// 原型基类
class Prototype
{
public:
    virtual ~Prototype() {}
    virtual std::unique_ptr<Prototype> clone() const = 0;
    virtual void operation() const = 0;
};

// 具体原型类
class ConcretePrototype : public Prototype
{
private:
    int data;

public:
    ConcretePrototype(int d) : data(d) {}

    // 克隆方法
    std::unique_ptr<Prototype> clone() const override
    {
        return std::make_unique<ConcretePrototype>(*this);
    }

    void operation() const override
    {
        std::cout << "ConcretePrototype operation with data: " << data << std::endl;
    }
};

int main()
{
    // 创建一个具体原型对象
    std::unique_ptr<Prototype> prototype = std::make_unique<ConcretePrototype>(10);

    // 克隆原型对象
    std::unique_ptr<Prototype> clonedPrototype = prototype->clone();

    // 调用原型和克隆对象的操作
    prototype->operation();
    clonedPrototype->operation();

    return 0;
}

/* 我们定义了一个Prototype基类，它有一个clone方法用于创建对象的副本。
   ConcretePrototype是Prototype的具体实现，它实现了clone方法来创建ConcretePrototype的一个新实例。

   在main函数中，我们创建了一个ConcretePrototype对象，并使用clone方法创建了它的一个副本。
   然后，我们调用了原型和克隆对象的operation方法，以验证它们都是有效的对象。*/

/*  1. clonedPrototype和prototype指向的是同一个对象吗？
    std::make_unique<ConcretePrototype>(*this) 创建了一个新的 ConcretePrototype 对象，并使用了拷贝构造函数来初始化它。
    因此，clonedPrototype 和 prototype 指向的是两个具有相同状态但在内存中位于不同位置的对象。

    2. 为什么我们使用make_unique而不是make_shared？
    make_shared将创建一个指向对象的shared_ptr，而make_unique将创建一个指向对象的unique_ptr。
    由于我们不需要在原型对象和克隆对象之间共享所有权，因此我们使用make_unique。

    3. 为什么我们使用unique_ptr而不是shared_ptr？
    我们使用unique_ptr，因为我们不需要在原型对象和克隆对象之间共享所有权。
*/

/* 原型模式的优点
    1. 原型模式允许你克隆现有的对象，而无需使代码依赖它们所属的类。
    2. 原型模式避免了对客户代码的复杂性，因为它将所有原型类集中在一个地方。
    3. 原型模式允许你动态配置客户代码，以便在运行时克隆不同的对象。

    原型模式的缺点
    1. 克隆包含循环引用的复杂对象可能非常麻烦。
    2. 有时，克隆并不比直接创建对象更方便。例如，你可以通过子类化原型并实现其clone方法来创建对象的预配置版本。
*/
