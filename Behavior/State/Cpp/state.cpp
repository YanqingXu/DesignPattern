#include <iostream>
#include <memory>

// 前向声明
class Context;

// State 接口
class State
{
public:
    virtual void handle(Context &context) = 0;
    virtual ~State() = default;
};

// Context 类
class Context
{
private:
    std::unique_ptr<State> state_;

public:
    void setState(std::unique_ptr<State> state)
    {
        state_ = std::move(state);
    }
    void request()
    {
        state_->handle(*this);
    }
};

// 具体状态类：Finished
class Finished : public State
{
public:
    void handle(Context &context) override
    {
        std::cout << "Finished state. No further state transitions." << std::endl;
    }
};

// 具体状态类：InProgress
class InProgress : public State
{
public:
    void handle(Context &context) override
    {
        std::cout << "InProgress state. Changing state to Finished." << std::endl;
        context.setState(std::make_unique<Finished>());
    }
};

// 具体状态类：Started
class Started : public State
{
public:
    void handle(Context &context) override
    {
        std::cout << "Started state. Changing state to InProgress." << std::endl;
        context.setState(std::make_unique<InProgress>());
    }
};

int main()
{
    Context context;

    // 设置初始状态为 Started
    context.setState(std::make_unique<Started>());

    // 处理请求，状态从 Started -> InProgress
    context.request();

    // 处理请求，状态从 InProgress -> Finished
    context.request();

    // 处理请求，状态为 Finished，不再变化
    context.request();

    return 0;
}
