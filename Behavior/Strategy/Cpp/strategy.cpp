#include <vector>
#include <iostream>
#include <algorithm>

// 策略接口声明了所有支持的算法的公共操作。
class SortingStrategy
{
public:
    virtual ~SortingStrategy() {}
    virtual void sort(std::vector<int> &dataset) = 0;
};

// 具体策略实现了冒泡排序算法
class BubbleSortStrategy : public SortingStrategy
{
public:
    void sort(std::vector<int> &dataset) override
    {
        std::cout << "Sorting using bubble sort.\n";
        // 实现冒泡排序算法
        for (size_t i = 0; i < dataset.size(); i++)
        {
            for (size_t j = 0; j < dataset.size() - i - 1; j++)
            {
                if (dataset[j] > dataset[j + 1])
                {
                    std::swap(dataset[j], dataset[j + 1]);
                }
            }
        }
    }
};

// 具体策略实现了快速排序算法
class QuickSortStrategy : public SortingStrategy
{
public:
    void sort(std::vector<int> &dataset) override
    {
        std::cout << "Sorting using quick sort.\n";
        // 实现快速排序算法，这里为了示例简单，直接调用标准库函数
        std::sort(dataset.begin(), dataset.end());
    }
};

// 上下文定义了客户端使用的接口。
class Context
{
private:
    SortingStrategy *strategy_;

public:
    Context(SortingStrategy *strategy = nullptr) : strategy_(strategy)
    {
    }

    ~Context()
    {
        delete this->strategy_;
    }

    void set_strategy(SortingStrategy *strategy)
    {
        delete this->strategy_;
        this->strategy_ = strategy;
    }

    void sort(std::vector<int> &dataset)
    {
        if (this->strategy_)
        {
            this->strategy_->sort(dataset);
        }
        else
        {
            std::cout << "Strategy not initialized.\n";
        }
    }
};

int main()
{
    std::vector<int> data = {23, 3, 5, 4, 9, 1};

    Context *context = new Context(new BubbleSortStrategy());
    context->sort(data); // 将会使用冒泡排序策略
    for (int i : data)
    {
        std::cout << i << " ";
    }
    std::cout << "\n";

    context->set_strategy(new QuickSortStrategy());
    context->sort(data); // 现在使用快速排序策略
    for (int i : data)
    {
        std::cout << i << " ";
    }
    std::cout << "\n";

    delete context;

    return 0;
}
