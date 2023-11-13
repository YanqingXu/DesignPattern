using System;

public abstract class CharacterState
{
    // 模板方法
    public void HandleState()
    {
        EnterState();
        PerformState();
        ExitState();
    }

    // 抽象方法：进入状态
    protected abstract void EnterState();

    // 抽象方法：执行状态逻辑
    protected abstract void PerformState();

    // 抽象方法：退出状态
    protected abstract void ExitState();
}

// 行走状态
public class WalkingState : CharacterState
{
    protected override void EnterState()
    {
        Console.WriteLine("Entering Walking State");
    }

    protected override void PerformState()
    {
        Console.WriteLine("Performing Walking State");
    }

    protected override void ExitState()
    {
        Console.WriteLine("Exiting Walking State");
    }
}

// 跳跃状态
public class JumpingState : CharacterState
{
    protected override void EnterState()
    {
        Console.WriteLine("Entering Jumping State");
    }

    protected override void PerformState()
    {
        Console.WriteLine("Performing Jumping State");
    }

    protected override void ExitState()
    {
        Console.WriteLine("Exiting Jumping State");
    }
}

class Program
{
    static void Main(string[] args)
    {
        CharacterState walking = new WalkingState();
        walking.HandleState();

        CharacterState jumping = new JumpingState();
        jumping.HandleState();
    }
}
