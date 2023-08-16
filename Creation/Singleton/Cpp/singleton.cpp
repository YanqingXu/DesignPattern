#include <iostream>
#include <mutex>

template <typename T>
class Singleton
{
private:
    static T *instance;
    static std::mutex mtx;

    // 私有构造函数确保外部不能创建实例
    Singleton() = default;

    // 禁止复制和赋值
    Singleton(const Singleton &) = delete;
    Singleton &operator=(const Singleton &) = delete;

public:
    static T &GetInstance()
    {
        if (!instance)
        {
            std::lock_guard<std::mutex> lock(mtx);
            if (!instance)
            {
                instance = new T();
            }
        }
        return *instance;
    }
};

// 静态成员初始化
template <typename T>
T *Singleton<T>::instance = nullptr;

template <typename T>
std::mutex Singleton<T>::mtx;

class MyClass
{
public:
    void Display()
    {
        std::cout << "MyClass instance!" << std::endl;
    }
};

int main()
{
    MyClass &instance1 = Singleton<MyClass>::GetInstance();
    MyClass &instance2 = Singleton<MyClass>::GetInstance();

    instance1.Display();
    instance2.Display();

    if (&instance1 == &instance2)
    {
        std::cout << "两个实例是相同的" << std::endl;
    }
    else
    {
        std::cout << "两个实例是不同的" << std::endl;
    }

    return 0;
}

/*
在这个实现中，Singleton 类使用了模板 T。我们使用了双重检查锁定模式（Double-Checked Locking Pattern）来确保线程安全性。
这种模式可以确保在多线程环境下，单例只被创建一次，并且创建单例的开销只在第一次访问时发生。
*/
