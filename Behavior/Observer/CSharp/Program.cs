using System;
using System.Collections.Generic;

// 成就观察者接口
public interface IAchievementObserver
{
    void OnAchievementUnlocked(string achievementName);
}

// 成就系统类
public class AchievementSystem
{
    private List<IAchievementObserver> observers = new List<IAchievementObserver>();
    private HashSet<string> unlockedAchievements = new HashSet<string>();

    // 注册观察者
    public void Subscribe(IAchievementObserver observer)
    {
        if (!observers.Contains(observer))
        {
            observers.Add(observer);
        }
    }

    // 取消注册观察者
    public void Unsubscribe(IAchievementObserver observer)
    {
        if (observers.Contains(observer))
        {
            observers.Remove(observer);
        }
    }

    // 解锁成就并通知观察者
    public void UnlockAchievement(string achievementName)
    {
        if (!unlockedAchievements.Contains(achievementName))
        {
            unlockedAchievements.Add(achievementName);
            NotifyObservers(achievementName);
        }
    }

    // 通知所有观察者
    private void NotifyObservers(string achievementName)
    {
        foreach (var observer in observers)
        {
            observer.OnAchievementUnlocked(achievementName);
        }
    }
}

// 具体观察者：用户界面组件
public class UIComponent : IAchievementObserver
{
    public void OnAchievementUnlocked(string achievementName)
    {
        Console.WriteLine($"UI Notification: Achievement Unlocked - {achievementName}!");
    }
}

class Program
{
    static void Main(string[] args)
    {
        // 创建成就系统
        AchievementSystem achievementSystem = new AchievementSystem();

        // 创建UI组件并注册到成就系统
        UIComponent uiComponent = new UIComponent();
        achievementSystem.Subscribe(uiComponent);

        // 解锁一个成就
        achievementSystem.UnlockAchievement("First Kill");
        // 输出：UI Notification: Achievement Unlocked - First Kill!

        // 如果不再需要通知UI组件，可以取消注册
        achievementSystem.Unsubscribe(uiComponent);

        // 继续解锁另一个成就
        achievementSystem.UnlockAchievement("First Win");
        // 由于UI组件已经取消注册，所以不会有输出
    }
}
