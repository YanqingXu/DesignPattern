#include <iostream>

// 子系统1：图形渲染
class GraphicsSubsystem
{
public:
    void initialize()
    {
        std::cout << "GraphicsSubsystem initialized" << std::endl;
    }

    void render()
    {
        std::cout << "Rendering graphics" << std::endl;
    }
};

// 子系统2：声音管理
class AudioSubsystem
{
public:
    void initialize()
    {
        std::cout << "AudioSubsystem initialized" << std::endl;
    }

    void playSound()
    {
        std::cout << "Playing sound" << std::endl;
    }
};

// 子系统3：物理引擎
class PhysicsSubsystem
{
public:
    void initialize()
    {
        std::cout << "PhysicsSubsystem initialized" << std::endl;
    }

    void simulate()
    {
        std::cout << "Simulating physics" << std::endl;
    }
};

// 游戏引擎外观
class GameEngineFacade
{
private:
    GraphicsSubsystem graphics;
    AudioSubsystem audio;
    PhysicsSubsystem physics;

public:
    void initialize()
    {
        graphics.initialize();
        audio.initialize();
        physics.initialize();
    }

    void startGame()
    {
        std::cout << "Starting the game" << std::endl;
    }

    void updateGame()
    {
        graphics.render();
        audio.playSound();
        physics.simulate();
    }

    void shutdown()
    {
        std::cout << "Shutting down the game" << std::endl;
    }
};

int main()
{
    // 创建游戏引擎外观
    GameEngineFacade gameEngine;

    // 初始化游戏引擎
    gameEngine.initialize();

    // 启动游戏
    gameEngine.startGame();

    // 游戏循环中更新游戏
    for (int i = 0; i < 5; ++i)
    {
        gameEngine.updateGame();
    }

    // 关闭游戏
    gameEngine.shutdown();

    return 0;
}
