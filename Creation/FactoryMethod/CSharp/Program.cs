using System;

namespace FactoryMethod
{
    public interface IProduct
    {
        void Show();
    }

    public class ConcreteProductA : IProduct
    {
        public void Show()
        {
            Console.WriteLine("ConcreteProductA...");
        }
    }

    public class ConcreteProductB : IProduct
    {
        public void Show()
        {
            Console.WriteLine("ConcreteProductB...");
        }
    }


    public interface IFactory
    {
        IProduct CreateProduct();
    }

    public class ConcreteFactoryA : IFactory
    {
        public IProduct CreateProduct()
        {
            return new ConcreteProductA();
        }
    }

    public class ConcreteFactoryB : IFactory
    {
        public IProduct CreateProduct()
        {
            return new ConcreteProductB();
        }
    }


    class Program
    {
        static void Main(string[] args)
        {
            IFactory factory = new ConcreteFactoryA();
            IProduct product = factory.CreateProduct();
            product.Show();

            factory = new ConcreteFactoryB();
            product = factory.CreateProduct();
            product.Show();
        }
    }
}