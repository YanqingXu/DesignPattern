using System;
using System.Collections.Generic;

// 中介者接口
public interface IMediator
{
    void Register(Colleague colleague);
    void Send(string message, Colleague colleague);
}

// 具体中介者类
public class ConcreteMediator : IMediator
{
    private List<Colleague> colleagues = new List<Colleague>();

    public void Register(Colleague colleague)
    {
        colleagues.Add(colleague);
    }

    public void Send(string message, Colleague colleague)
    {
        foreach (var coll in colleagues)
        {
            // 不要将消息发送给自己，而是发送给其他同事
            if (coll != colleague)
            {
                coll.HandleNotification(message);
            }
        }
    }
}

// 同事抽象类
public abstract class Colleague
{
    protected IMediator Mediator;

    public Colleague(IMediator mediator)
    {
        this.Mediator = mediator;
    }

    public abstract void HandleNotification(string message);
    public void Send(string message)
    {
        Mediator.Send(message, this);
    }
}

// 具体同事类
public class ConcreteColleague1 : Colleague
{
    public ConcreteColleague1(IMediator mediator) : base(mediator) { }

    public override void HandleNotification(string message)
    {
        Console.WriteLine($"Colleague1 receives message: {message}");
    }
}

public class ConcreteColleague2 : Colleague
{
    public ConcreteColleague2(IMediator mediator) : base(mediator) { }

    public override void HandleNotification(string message)
    {
        Console.WriteLine($"Colleague2 receives message: {message}");
    }
}

// 客户端代码
public class Program
{
    public static void Main()
    {
        // 创建中介者
        ConcreteMediator mediator = new ConcreteMediator();

        // 创建同事对象并注册到中介者
        Colleague colleague1 = new ConcreteColleague1(mediator);
        Colleague colleague2 = new ConcreteColleague2(mediator);
        mediator.Register(colleague1);
        mediator.Register(colleague2);

        // 发送消息
        colleague1.Send("Hello, World!");
        colleague2.Send("Hi there!");
    }
}
