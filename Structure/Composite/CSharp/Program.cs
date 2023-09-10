using System;
using System.Collections.Generic;

// 抽象基类表示游戏任务
abstract class GameTask
{
    protected string name;

    public GameTask(string name)
    {
        this.name = name;
    }

    public abstract void Add(GameTask task);
    public abstract void Remove(GameTask task);
    public abstract void Display(int depth);
}

// 叶子节点：具体游戏任务
class ConcreteTask : GameTask
{
    public ConcreteTask(string name) : base(name)
    {
    }

    public override void Add(GameTask task)
    {
        Console.WriteLine("Cannot add subtasks to a concrete task.");
    }

    public override void Remove(GameTask task)
    {
        Console.WriteLine("Cannot remove subtasks from a concrete task.");
    }

    public override void Display(int depth)
    {
        Console.WriteLine(new string('-', depth) + name);
    }
}

// 组合节点：游戏任务组合
class TaskComposite : GameTask
{
    private List<GameTask> subtasks = new List<GameTask>();

    public TaskComposite(string name) : base(name)
    {
    }

    public override void Add(GameTask task)
    {
        subtasks.Add(task);
    }

    public override void Remove(GameTask task)
    {
        subtasks.Remove(task);
    }

    public override void Display(int depth)
    {
        Console.WriteLine(new string('-', depth) + name);
        foreach (var task in subtasks)
        {
            task.Display(depth + 2);
        }
    }
}

class Program
{
    static void Main(string[] args)
    {
        // 创建游戏任务及子任务
        var mainTask = new TaskComposite("Main Quest");
        var subTask1 = new TaskComposite("Subtask 1");
        var subTask2 = new TaskComposite("Subtask 2");
        var leafTask1 = new ConcreteTask("Collect Items");
        var leafTask2 = new ConcreteTask("Defeat Boss");

        // 组织任务层次结构
        mainTask.Add(subTask1);
        mainTask.Add(subTask2);
        subTask1.Add(leafTask1);
        subTask2.Add(leafTask2);

        // 显示任务层次结构
        mainTask.Display(0);
    }
}
