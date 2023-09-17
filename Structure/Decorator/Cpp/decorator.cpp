#include <iostream>
#include <string>

// 基本任务类
class Task
{
public:
    Task(const std::string &name) : name(name) {}

    virtual void execute()
    {
        std::cout << "Executing task: " << name << std::endl;
    }

private:
    std::string name;
};

// 装饰器基类
class TaskDecorator : public Task
{
public:
    TaskDecorator(Task *task) : Task("task_decorator")
    {
        this->task = task;
    }

    void execute() override
    {
        task->execute();
    }

private:
    Task *task;
};

// 具体装饰器 - 添加条件
class ConditionalTaskDecorator : public TaskDecorator
{
public:
    ConditionalTaskDecorator(Task *task, const std::string &condition)
        : TaskDecorator(task), condition(condition) {}

    void execute() override
    {
        std::cout << "Checking condition: " << condition << std::endl;
        TaskDecorator::execute();
    }

private:
    std::string condition;
};

// 具体装饰器 - 添加奖励
class RewardTaskDecorator : public TaskDecorator
{
public:
    RewardTaskDecorator(Task *task, int reward)
        : TaskDecorator(task), reward(reward) {}

    void execute() override
    {
        TaskDecorator::execute();
        std::cout << "Adding reward: " << reward << " points" << std::endl;
    }

private:
    int reward;
};

int main()
{
    // 创建基本任务
    Task *basicTask = new Task("Basic Task");

    // 使用装饰器添加条件和奖励
    Task *taskWithCondition = new ConditionalTaskDecorator(basicTask, "Level >= 10");
    Task *taskWithReward = new RewardTaskDecorator(taskWithCondition, 50);

    // 执行任务
    taskWithReward->execute();

    // 释放资源
    delete taskWithReward;
    return 0;
}
