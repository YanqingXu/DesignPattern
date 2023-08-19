trait Product {
    fn show(&self);
}

struct ConcreteProductA {}

impl Product for ConcreteProductA {
    fn show(&self) {
        println!("ConcreteProductA");
    }
}

struct ConcreteProductB {}

impl Product for ConcreteProductB {
    fn show(&self) {
        println!("ConcreteProductB");
    }
}

trait Factory {
    fn create_product(&self) -> Box<dyn Product>;
}

struct ConcreteFactoryA {}

impl Factory for ConcreteFactoryA {
    fn create_product(&self) -> Box<dyn Product> {
        Box::new(ConcreteProductA {})
    }
}

struct ConcreteFactoryB {}

impl Factory for ConcreteFactoryB {
    fn create_product(&self) -> Box<dyn Product> {
        Box::new(ConcreteProductB {})
    }
}

fn main() {
    let factory_a: Box<dyn Factory> = Box::new(ConcreteFactoryA {});
    let product_a = factory_a.create_product();
    product_a.show();

    let factory_b: Box<dyn Factory> = Box::new(ConcreteFactoryB {});
    let product_b = factory_b.create_product();
    product_b.show();
}
