trait Implementor {
    fn operation_impl(&self) -> String;
}

struct ConcreteImplementorA;

impl Implementor for ConcreteImplementorA {
    fn operation_impl(&self) -> String {
        "ConcreteImplementorA: operation_impl".to_string()
    }
}

struct ConcreteImplementorB;

impl Implementor for ConcreteImplementorB {
    fn operation_impl(&self) -> String {
        "ConcreteImplementorB: operation_impl".to_string()
    }
}

trait Abstraction {
    fn operation(&self) -> String;
}

struct RefinedAbstraction {
    implementor: Box<dyn Implementor>,
}

impl RefinedAbstraction {
    fn new(implementor: Box<dyn Implementor>) -> Self {
        RefinedAbstraction { implementor }
    }
}

impl Abstraction for RefinedAbstraction {
    fn operation(&self) -> String {
        format!("RefinedAbstraction: {}", self.implementor.operation_impl())
    }
}

fn main() {
    let implementor_a = Box::new(ConcreteImplementorA);
    let implementor_b = Box::new(ConcreteImplementorB);

    let abstraction_a = RefinedAbstraction::new(implementor_a);
    let abstraction_b = RefinedAbstraction::new(implementor_b);

    println!("{}", abstraction_a.operation());
    println!("{}", abstraction_b.operation());
}
