interface IProduct {
    show(): void;
}

class ConcreteProductA implements IProduct {
    public show(): void {
        console.log('ConcreteProductA');
    }
}

class ConcreteProductB implements IProduct {
    public show(): void {
        console.log('ConcreteProductB');
    }
}

abstract class Factory {
    public abstract createProduct(): IProduct;
}

class ConcreteFactoryA extends Factory {
    public createProduct(): IProduct {
        return new ConcreteProductA();
    }
}

class ConcreteFactoryB extends Factory {
    public createProduct(): IProduct {
        return new ConcreteProductB();
    }
}

let factory: Factory = new ConcreteFactoryA();
const productA = factory.createProduct();
productA.show();

factory = new ConcreteFactoryB();
const productB = factory.createProduct();
productB.show();