#include <iostream>

// 抽象产品：Button
class Button
{
public:
    virtual void paint() = 0;
};

// 抽象产品：Checkbox
class Checkbox
{
public:
    virtual void paint() = 0;
};

// 具体产品：WindowsButton
class WindowsButton : public Button
{
public:
    void paint() override
    {
        std::cout << "Windows Button painted." << std::endl;
    }
};

// 具体产品：WindowsCheckbox
class WindowsCheckbox : public Checkbox
{
public:
    void paint() override
    {
        std::cout << "Windows Checkbox painted." << std::endl;
    }
};

// 具体产品：MacButton
class MacButton : public Button
{
public:
    void paint() override
    {
        std::cout << "Mac Button painted." << std::endl;
    }
};

// 具体产品：MacCheckbox
class MacCheckbox : public Checkbox
{
public:
    void paint() override
    {
        std::cout << "Mac Checkbox painted." << std::endl;
    }
};

// 抽象工厂
class GUIFactory
{
public:
    virtual Button *createButton() = 0;
    virtual Checkbox *createCheckbox() = 0;
};

// 具体工厂：WindowsFactory
class WindowsFactory : public GUIFactory
{
public:
    Button *createButton() override
    {
        return new WindowsButton();
    }

    Checkbox *createCheckbox() override
    {
        return new WindowsCheckbox();
    }
};

// 具体工厂：MacFactory
class MacFactory : public GUIFactory
{
public:
    Button *createButton() override
    {
        return new MacButton();
    }

    Checkbox *createCheckbox() override
    {
        return new MacCheckbox();
    }
};

int main()
{
    GUIFactory *factory = nullptr;
    Button *button = nullptr;
    Checkbox *checkbox = nullptr;

    // 使用Windows工厂
    factory = new WindowsFactory();
    button = factory->createButton();
    checkbox = factory->createCheckbox();

    button->paint();
    checkbox->paint();

    delete button;
    delete checkbox;
    delete factory;

    // 使用Mac工厂
    factory = new MacFactory();
    button = factory->createButton();
    checkbox = factory->createCheckbox();

    button->paint();
    checkbox->paint();

    delete button;
    delete checkbox;
    delete factory;

    return 0;
}
