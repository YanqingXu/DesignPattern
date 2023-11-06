#include <iostream>
#include <list>

// 观察者基类
class Observer
{
public:
    virtual ~Observer() {}
    virtual void update(int score) = 0;
};

// 被观察的主题基类
class Subject
{
private:
    std::list<Observer *> observers;

public:
    virtual ~Subject() {}

    void attach(Observer *obs)
    {
        observers.push_back(obs);
    }

    void detach(Observer *obs)
    {
        observers.remove(obs);
    }

    void notify(int score)
    {
        for (Observer *obs : observers)
        {
            obs->update(score);
        }
    }
};

// 具体主题：游戏记分板
class ScoreBoard : public Subject
{
private:
    int score;

public:
    ScoreBoard() : score(0) {}

    void setScore(int newScore)
    {
        score = newScore;
        notify(score); // 当分数变化时，通知所有观察者
    }

    int getScore() const
    {
        return score;
    }
};

// 具体观察者：游戏UI组件，显示分数
class GameUI : public Observer
{
public:
    virtual void update(int score) override
    {
        std::cout << "Game UI Updated: Score is " << score << std::endl;
    }
};

int main()
{
    // 创建主题和观察者
    ScoreBoard scoreboard;
    GameUI gameUI;

    // 注册观察者到主题
    scoreboard.attach(&gameUI);

    // 改变分数，观察UI更新
    scoreboard.setScore(10); // 输出: Game UI Updated: Score is 10
    scoreboard.setScore(20); // 输出: Game UI Updated: Score is 20

    // 如果不再需要UI更新，可以解除注册
    scoreboard.detach(&gameUI);

    // 继续改变分数，UI不再更新
    scoreboard.setScore(30); // 无输出

    return 0;
}
