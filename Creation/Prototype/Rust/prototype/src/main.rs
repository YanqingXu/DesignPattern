trait Prototype {
    fn clone_box(&self) -> Box<dyn Prototype>;
    fn operation(&self);
}

#[derive(Clone)]
struct ConcretePrototype {
    data: i32,
}

impl Prototype for ConcretePrototype {
    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }

    fn operation(&self) {
        println!("ConcretePrototype operation with data: {}", self.data);
    }
}

impl ConcretePrototype {
    fn new(data: i32) -> Self {
        ConcretePrototype { data }
    }
}

fn main() {
    // 创建一个具体原型对象
    let prototype: Box<dyn Prototype> = Box::new(ConcretePrototype::new(10));

    // 克隆原型对象
    let cloned_prototype = prototype.clone_box();

    // 调用原型和克隆对象的操作
    prototype.operation();
    cloned_prototype.operation();
}

/*
我们定义了一个Prototype trait，它有一个clone_box方法用于创建对象的副本。
ConcretePrototype结构体实现了这个trait，并提供了clone_box方法的具体实现。

在main函数中，我们创建了一个ConcretePrototype对象，并使用clone_box方法创建了它的一个副本。
然后，我们调用了原型和克隆对象的operation方法，以验证它们都是有效的对象。

这个简单的例子展示了如何使用Rust来实现原型模式，创建对象的深拷贝。
*/
