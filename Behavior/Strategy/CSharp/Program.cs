public interface IBehaviorStrategy
{
    void ExecuteBehavior();
}

public class AggressiveBehavior : IBehaviorStrategy
{
    public void ExecuteBehavior()
    {
        Console.WriteLine("AI is attacking aggressively!");
    }
}

public class DefensiveBehavior : IBehaviorStrategy
{
    public void ExecuteBehavior()
    {
        Console.WriteLine("AI is defending!");
    }
}

public class PassiveBehavior : IBehaviorStrategy
{
    public void ExecuteBehavior()
    {
        Console.WriteLine("AI is passive and observing.");
    }
}

public class AICharacter
{
    private IBehaviorStrategy _behaviorStrategy;

    public AICharacter(IBehaviorStrategy behaviorStrategy)
    {
        _behaviorStrategy = behaviorStrategy;
    }

    public void SetStrategy(IBehaviorStrategy behaviorStrategy)
    {
        _behaviorStrategy = behaviorStrategy;
    }

    public void PerformAction()
    {
        _behaviorStrategy.ExecuteBehavior();
    }
}

class Program
{
    static void Main(string[] args)
    {
        // 创建AI角色并设置初始行为
        AICharacter aiCharacter = new AICharacter(new PassiveBehavior());
        aiCharacter.PerformAction();  // 输出: AI is passive and observing.

        // 改变AI行为为攻击
        aiCharacter.SetStrategy(new AggressiveBehavior());
        aiCharacter.PerformAction();  // 输出: AI is attacking aggressively!

        // 改变AI行为为防御
        aiCharacter.SetStrategy(new DefensiveBehavior());
        aiCharacter.PerformAction();  // 输出: AI is defending!
    }
}
