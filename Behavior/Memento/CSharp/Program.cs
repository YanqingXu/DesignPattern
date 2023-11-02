using System;
using System.Collections.Generic;

// Memento类存储原始对象的内部状态
public class Memento
{
    public string State { get; private set; }

    public Memento(string state)
    {
        State = state;
    }
}

// Originator类可以创建一个备忘录，并从备忘录中恢复其状态
public class Originator
{
    public string State { get; set; }

    public Memento CreateMemento()
    {
        return new Memento(State);
    }

    public void SetMemento(Memento memento)
    {
        State = memento.State;
    }
}

// Caretaker负责保存备忘录，但是不会对备忘录的内容进行操作或检查
public class Caretaker
{
    private List<Memento> mementoList = new List<Memento>();

    public void AddMemento(Memento memento)
    {
        mementoList.Add(memento);
    }

    public Memento GetMemento(int index)
    {
        return mementoList[index];
    }
}

public class Program
{
    public static void Main()
    {
        Originator originator = new Originator();
        Caretaker caretaker = new Caretaker();

        originator.State = "State 1";
        caretaker.AddMemento(originator.CreateMemento());

        originator.State = "State 2";
        caretaker.AddMemento(originator.CreateMemento());

        // 恢复到状态1
        originator.SetMemento(caretaker.GetMemento(0));
        Console.WriteLine("Current State: " + originator.State);

        // 恢复到状态2
        originator.SetMemento(caretaker.GetMemento(1));
        Console.WriteLine("Current State: " + originator.State);
    }
}
