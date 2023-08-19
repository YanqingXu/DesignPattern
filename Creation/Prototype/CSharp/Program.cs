using System;

namespace PrototypePattern
{
    // Prototype 接口
    public interface IPrototype
    {
        IPrototype Clone();
        void Operation();
    }

    // ConcretePrototype 类
    public class ConcretePrototype : IPrototype
    {
        private int _data;

        public ConcretePrototype(int data)
        {
            _data = data;
        }

        // 实现 Clone 方法
        public IPrototype Clone()
        {
            return new ConcretePrototype(_data);
        }

        // 实现 Operation 方法
        public void Operation()
        {
            Console.WriteLine($"ConcretePrototype operation with data: {_data}");
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            // 创建一个具体原型对象
            IPrototype prototype = new ConcretePrototype(10);

            // 克隆原型对象
            IPrototype clonedPrototype = prototype.Clone();

            // 调用原型和克隆对象的操作
            prototype.Operation();
            clonedPrototype.Operation();

            Console.ReadKey();
        }
    }
}

/*
    我们定义了一个IPrototype接口，它有一个Clone方法用于创建对象的副本。
    ConcretePrototype类实现了这个接口，并提供了Clone方法的具体实现。

    在Main方法中，我们创建了一个ConcretePrototype对象，并使用Clone方法创建了它的一个副本。
    然后，我们调用了原型和克隆对象的Operation方法，以验证它们都是有效的对象。
*/
