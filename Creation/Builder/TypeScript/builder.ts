class Car {
    Wheels?: number;
    Engine?: string;
    Color?: string;
}

class CarBuilder {
    private car: Car;

    constructor() {
        this.car = new Car();
    }

    setWheels(wheels: number): CarBuilder {
        this.car.Wheels = wheels;
        return this;
    }

    setEngine(engine: string): CarBuilder {
        this.car.Engine = engine;
        return this;
    }

    setColor(color: string): CarBuilder {
        this.car.Color = color;
        return this;
    }

    build(): Car {
        return this.car;
    }
}

function main() {
    const car = new CarBuilder()
        .setWheels(4)
        .setEngine("V8")
        .setColor("Red")
        .build();

    console.log(car);
}

main();