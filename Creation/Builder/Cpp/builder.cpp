#include <iostream>
#include <string>

// 1. Product
class Car
{
private:
    int wheels;
    std::string color;
    std::string engine;

public:
    void setWheels(int w) { wheels = w; }
    void setColor(const std::string &c) { color = c; }
    void setEngine(const std::string &e) { engine = e; }

    void showDetails() const
    {
        std::cout << "Car with " << wheels << " wheels, color " << color << " and " << engine << " engine." << std::endl;
    }
};

// 2. Builder
class CarBuilder
{
public:
    virtual ~CarBuilder() {}
    virtual void setWheels(int w) = 0;
    virtual void setColor(const std::string &c) = 0;
    virtual void setEngine(const std::string &e) = 0;
    virtual Car *build() = 0;
};

// 3. ConcreteBuilder
class SedanCarBuilder : public CarBuilder
{
private:
    Car car;

public:
    void setWheels(int w) override
    {
        car.setWheels(w);
    }

    void setColor(const std::string &c) override
    {
        car.setColor(c);
    }

    void setEngine(const std::string &e) override
    {
        car.setEngine(e);
    }

    Car *build() override
    {
        return &car;
    }
};

// 4. Director
class ManufacturingDirector
{
private:
    CarBuilder *builder;

public:
    void setBuilder(CarBuilder *b)
    {
        builder = b;
    }

    void construct()
    {
        builder->setWheels(4);
        builder->setColor("Red");
        builder->setEngine("V6");
    }
};

int main()
{
    // Client code
    ManufacturingDirector director;
    SedanCarBuilder sedanBuilder;

    director.setBuilder(&sedanBuilder);
    director.construct();

    Car *car = sedanBuilder.build();
    car->showDetails();

    return 0;
}
