interface Prototype {
    clone(): Prototype;
    operation(): void;
}

class ConcretePrototype implements Prototype {
    private data: number;

    constructor(data: number) {
        this.data = data;
    }

    clone(): Prototype {
        return new ConcretePrototype(this.data);
    }

    operation(): void {
        console.log(`ConcretePrototype operation with data: ${this.data}`);
    }
}

// 使用示例
const prototype: Prototype = new ConcretePrototype(10);
const clonedPrototype: Prototype = prototype.clone();

prototype.operation();
clonedPrototype.operation();

/*
    我们定义了一个Prototype接口，它有一个clone方法用于创建对象的副本。
    ConcretePrototype类实现了这个接口，并提供了clone方法的具体实现。

    在使用示例中，我们创建了一个ConcretePrototype对象，并使用clone方法创建了它的一个副本。
    然后，我们调用了原型和克隆对象的operation方法，以验证它们都是有效的对象。
*/
