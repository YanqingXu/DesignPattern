class Singleton<T> {
    private static instances: Map<any, any> = new Map();

    private constructor() { }

    public static getInstance<T>(this: new () => T): T {
        if (!Singleton.instances.get(this)) {
            Singleton.instances.set(this, new this());
        }
        return Singleton.instances.get(this) as T;
    }
}

class MyClass {
    public display() {
        console.log("MyClass instance!");
    }
}

const instance1 = Singleton.getInstance(MyClass);
const instance2 = Singleton.getInstance(MyClass);

instance1.display();
instance2.display();

if (instance1 === instance2) {
    console.log("两个实例是相同的");
} else {
    console.log("两个实例是不同的");
}
