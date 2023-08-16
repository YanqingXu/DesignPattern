using System;

public class Singleton<T> where T : new()
{
    private static readonly Lazy<T> instance = new Lazy<T>(() => new T());

    public static T Instance
    {
        get { return instance.Value; }
    }

    // 确保外部不能创建实例
    protected Singleton() { }
}

public class MyClass
{
    public void Display()
    {
        Console.WriteLine("MyClass instance!");
    }
}

public class Program
{
    public static void Main()
    {
        var instance1 = Singleton<MyClass>.Instance;
        var instance2 = Singleton<MyClass>.Instance;

        instance1.Display();
        instance2.Display();

        if (ReferenceEquals(instance1, instance2))
        {
            Console.WriteLine("两个实例是相同的");
        }
        else
        {
            Console.WriteLine("两个实例是不同的");
        }
    }
}
