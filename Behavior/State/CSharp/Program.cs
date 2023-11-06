using System;

// State 接口
public interface IState
{
    void Handle(Document context);
}

// Context 类
public class Document
{
    private IState _state;

    public Document(IState state)
    {
        TransitionTo(state);
    }

    public void TransitionTo(IState state)
    {
        Console.WriteLine($"Document has changed its state to {state.GetType().Name}.");
        _state = state;
    }

    public void Request()
    {
        _state.Handle(this);
    }
}

// 具体状态类：Draft
public class Draft : IState
{
    public void Handle(Document context)
    {
        Console.WriteLine("Draft state. Changing state to Review.");
        context.TransitionTo(new Review());
    }
}

// 具体状态类：Review
public class Review : IState
{
    public void Handle(Document context)
    {
        Console.WriteLine("Review state. Changing state to Published.");
        context.TransitionTo(new Published());
    }
}

// 具体状态类：Published
public class Published : IState
{
    public void Handle(Document context)
    {
        Console.WriteLine("Published state. The document is now in its final state.");
    }
}

class Program
{
    static void Main(string[] args)
    {
        var document = new Document(new Draft());

        // 处理请求，状态从 Draft -> Review
        document.Request();

        // 处理请求，状态从 Review -> Published
        document.Request();

        // 处理请求，状态为 Published，不再变化
        document.Request();
    }
}
