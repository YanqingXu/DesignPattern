// 定义行为枚举
enum Behavior {
    Success,
    Failure,
    Running,
}

// 定义节点 trait
trait Node {
    fn tick(&mut self) -> Behavior;
}

// 定义 Action 结构体
struct Action {
    action: Box<dyn FnMut() -> Behavior>,
}

impl Node for Action {
    fn tick(&mut self) -> Behavior {
        (self.action)()
    }
}

// 定义 Condition 结构体
struct Condition {
    condition: Box<dyn FnMut() -> bool>,
}

impl Node for Condition {
    fn tick(&mut self) -> Behavior {
        if (self.condition)() {
            Behavior::Success
        } else {
            Behavior::Failure
        }
    }
}

// 定义 Sequence 结构体
struct Sequence {
    children: Vec<Box<dyn Node>>,
}

impl Node for Sequence {
    fn tick(&mut self) -> Behavior {
        for child in &mut self.children {
            match child.tick() {
                Behavior::Failure => return Behavior::Failure,
                Behavior::Running => return Behavior::Running,
                Behavior::Success => continue,
            }
        }
        Behavior::Success
    }
}

// 定义 Selector 结构体
struct Selector {
    children: Vec<Box<dyn Node>>,
}

impl Node for Selector {
    fn tick(&mut self) -> Behavior {
        for child in &mut self.children {
            match child.tick() {
                Behavior::Success => return Behavior::Success,
                Behavior::Running => return Behavior::Running,
                Behavior::Failure => continue,
            }
        }
        Behavior::Failure
    }
}

// 主函数
fn main() {
    let condition = Condition {
        condition: Box::new(|| true),
    };

    let action = Action {
        action: Box::new(|| {
            println!("Action executed!");
            Behavior::Success
        }),
    };

    let mut sequence = Sequence {
        children: vec![Box::new(condition), Box::new(action)],
    };

    match sequence.tick() {
        Behavior::Success => println!("Sequence succeeded"),
        Behavior::Failure => println!("Sequence failed"),
        Behavior::Running => println!("Sequence is running"),
    }
}
