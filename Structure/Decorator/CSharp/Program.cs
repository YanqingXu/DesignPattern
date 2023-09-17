using System;

// 基础组件接口
public interface ICar
{
    void Assemble();
}

// 基础组件
public class Car : ICar
{
    public void Assemble()
    {
        Console.WriteLine("组装基本汽车");
    }
}

// 装饰器基类
public abstract class CarDecorator : ICar
{
    protected ICar decoratedCar;

    public CarDecorator(ICar car)
    {
        decoratedCar = car;
    }

    public virtual void Assemble()
    {
        decoratedCar.Assemble();
    }
}

// 具体装饰器类
public class GPSDecorator : CarDecorator
{
    public GPSDecorator(ICar car) : base(car)
    {
    }

    public override void Assemble()
    {
        base.Assemble();
        Console.WriteLine("添加GPS功能");
    }
}

class Program
{
    static void Main(string[] args)
    {
        // 创建基本汽车
        ICar car = new Car();

        // 添加GPS功能装饰器
        ICar carWithGPS = new GPSDecorator(car);

        // 组装汽车
        carWithGPS.Assemble();
    }
}
