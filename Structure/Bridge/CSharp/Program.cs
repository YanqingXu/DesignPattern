using System;

// 抽象类：Character
public abstract class Character
{
    protected IBehavior behavior;
    protected IAttribute attribute;

    public Character(IBehavior behavior, IAttribute attribute)
    {
        this.behavior = behavior;
        this.attribute = attribute;
    }

    public abstract void PerformAction();
}

// 具体实现接口：IBehavior
public interface IBehavior
{
    void Execute();
}

// 具体实现接口：IAttribute
public interface IAttribute
{
    void Display();
}

// 具体实现类：AttackBehavior
public class AttackBehavior : IBehavior
{
    public void Execute()
    {
        Console.WriteLine("Attacking the enemy.");
    }
}

// 具体实现类：DefendBehavior
public class DefendBehavior : IBehavior
{
    public void Execute()
    {
        Console.WriteLine("Defending against the enemy.");
    }
}

// 具体实现类：HighAttribute
public class HighAttribute : IAttribute
{
    public void Display()
    {
        Console.WriteLine("High-level attributes.");
    }
}

// 具体实现类：LowAttribute
public class LowAttribute : IAttribute
{
    public void Display()
    {
        Console.WriteLine("Low-level attributes.");
    }
}

// 具体扩展类：Warrior
public class Warrior : Character
{
    public Warrior(IBehavior behavior, IAttribute attribute) : base(behavior, attribute) { }

    public override void PerformAction()
    {
        Console.Write("Warrior is ");
        behavior.Execute();
        attribute.Display();
    }
}

// 具体扩展类：Mage
public class Mage : Character
{
    public Mage(IBehavior behavior, IAttribute attribute) : base(behavior, attribute) { }

    public override void PerformAction()
    {
        Console.Write("Mage is ");
        behavior.Execute();
        attribute.Display();
    }
}

// 客户端代码
class Program
{
    static void Main(string[] args)
    {
        Character warrior = new Warrior(new AttackBehavior(), new HighAttribute());
        Character mage = new Mage(new DefendBehavior(), new LowAttribute());

        warrior.PerformAction();
        mage.PerformAction();
    }
}
