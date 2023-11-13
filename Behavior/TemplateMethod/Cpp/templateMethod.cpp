#include <iostream>

// 抽象基类
class GameLevel
{
public:
    // 模板方法
    void playLevel()
    {
        initialize();
        startPlay();
        endPlay();
    }

    // 默认实现，可被重写
    virtual void initialize()
    {
        std::cout << "Default Initialization" << std::endl;
    }

    // 纯虚函数，必须在子类中实现
    virtual void startPlay() = 0;
    virtual void endPlay() = 0;

    // 虚析构函数，确保正确的析构
    virtual ~GameLevel() {}
};

// 第一个关卡
class Level1 : public GameLevel
{
public:
    void initialize() override
    {
        std::cout << "Level 1 Initialization" << std::endl;
    }

    void startPlay() override
    {
        std::cout << "Starting Level 1" << std::endl;
    }

    void endPlay() override
    {
        std::cout << "Ending Level 1" << std::endl;
    }
};

// 第二个关卡
class Level2 : public GameLevel
{
public:
    void initialize() override
    {
        std::cout << "Level 2 Initialization" << std::endl;
    }

    void startPlay() override
    {
        std::cout << "Starting Level 2" << std::endl;
    }

    void endPlay() override
    {
        std::cout << "Ending Level 2" << std::endl;
    }
};

int main()
{
    GameLevel *level1 = new Level1();
    level1->playLevel();
    delete level1;

    GameLevel *level2 = new Level2();
    level2->playLevel();
    delete level2;

    return 0;
}
