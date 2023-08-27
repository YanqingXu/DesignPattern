interface Implementor {
    operationImpl(): void;
}

class Abstraction {
    protected implementor: Implementor;

    constructor(implementor: Implementor) {
        this.implementor = implementor;
    }

    operation(): void {
        this.implementor.operationImpl();
    }
}

class RefinedAbstraction extends Abstraction {
    constructor(implementor: Implementor) {
        super(implementor);
    }
}

class ConcreteImplementorA implements Implementor {
    operationImpl(): void {
        console.log("ConcreteImplementorA operation");
    }
}

class ConcreteImplementorB implements Implementor {
    operationImpl(): void {
        console.log("ConcreteImplementorB operation");
    }
}

const absA = new RefinedAbstraction(new ConcreteImplementorA());
absA.operation();

const absB = new RefinedAbstraction(new ConcreteImplementorB());
absB.operation();
