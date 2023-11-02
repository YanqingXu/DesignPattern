#include <iostream>
#include <vector>

// 迭代器基类
template <typename T>
class Iterator
{
public:
    virtual ~Iterator() {}
    virtual T next() = 0;
    virtual bool hasNext() const = 0;
};

// 容器接口
template <typename T>
class Container
{
public:
    virtual ~Container() {}
    virtual Iterator<T> *createIterator() const = 0;
    // 其他容器相关方法
};

// 具体迭代器
template <typename T>
class ConcreteIterator : public Iterator<T>
{
private:
    const std::vector<T> &collection;
    size_t current = 0;

public:
    ConcreteIterator(const std::vector<T> &collection) : collection(collection) {}

    T next() override
    {
        return collection[current++];
    }

    bool hasNext() const override
    {
        return current < collection.size();
    }
};

// 具体容器
template <typename T>
class ConcreteContainer : public Container<T>
{
private:
    std::vector<T> elements;

public:
    void add(T element)
    {
        elements.push_back(element);
    }

    Iterator<T> *createIterator() const override
    {
        return new ConcreteIterator<T>(elements);
    }

    // 可以添加其他容器相关的方法
};

int main()
{
    ConcreteContainer<int> container;
    container.add(1);
    container.add(2);
    container.add(3);

    Iterator<int> *it = container.createIterator();
    while (it->hasNext())
    {
        std::cout << it->next() << std::endl;
    }

    delete it; // 注意：使用原始指针需要手动管理内存
    return 0;
}
