#include <vector>
#include <iostream>

// Composite Pattern

class Component
{
public:
    virtual void operation() = 0;
};

class Leaf : public Component
{
public:
    void operation() override
    {
        // Leaf operation implementation
        std::cout << "Leaf operation" << std::endl;
    }
};

class Composite : public Component
{
private:
    std::vector<Component *> components;

public:
    void add(Component *component)
    {
        components.push_back(component);
    }

    void remove(Component *component)
    {
        // Remove component implementation
    }

    void operation() override
    {
        for (Component *component : components)
        {
            component->operation();
        }
    }
};

int main()
{
    Composite composite;
    composite.add(new Leaf());
    composite.add(new Leaf());
    composite.add(new Leaf());
    composite.operation();
}
