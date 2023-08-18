using System;

namespace BuilderPattern
{
    // 1. Product
    public class Car
    {
        public int Wheels { get; set; }
        public string? Color { get; set; }
        public string? Engine { get; set; }

        public void ShowDetails()
        {
            Console.WriteLine($"Car with {Wheels} wheels, color {Color} and {Engine} engine.");
        }
    }

    // 2. Builder
    public abstract class CarBuilder
    {
        protected Car car = new Car();

        public abstract CarBuilder SetWheels(int wheels);
        public abstract CarBuilder SetColor(string color);
        public abstract CarBuilder SetEngine(string engine);
        public abstract Car Build();
    }

    // 3. ConcreteBuilder
    public class SedanCarBuilder : CarBuilder
    {
        public override CarBuilder SetWheels(int wheels)
        {
            car.Wheels = wheels;
            return this;
        }

        public override CarBuilder SetColor(string color)
        {
            car.Color = color;
            return this;
        }

        public override CarBuilder SetEngine(string engine)
        {
            car.Engine = engine;
            return this;
        }

        public override Car Build()
        {
            return car;
        }
    }

    // 4. Director
    public class ManufacturingDirector
    {
        private CarBuilder? builder;

        public void SetBuilder(CarBuilder builder)
        {
            this.builder = builder;
        }

        public void Construct()
        {
            builder?.SetWheels(4)?.SetColor("Red")?.SetEngine("V6");
        }
    }

    class Program
    {
        static void Main(string[] args)
        {
            // Client code
            ManufacturingDirector director = new ManufacturingDirector();
            SedanCarBuilder sedanBuilder = new SedanCarBuilder();

            director.SetBuilder(sedanBuilder);
            director.Construct();

            Car car = sedanBuilder.Build();
            car.ShowDetails();
        }
    }
}
