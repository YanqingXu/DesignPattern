// 定义命令trait
trait Command {
    fn execute(&self);
}

// 创建两个结构体实现命令trait
struct PrintHello;
impl Command for PrintHello {
    fn execute(&self) {
        println!("Hello");
    }
}

struct PrintGoodbye;
impl Command for PrintGoodbye {
    fn execute(&self) {
        println!("Goodbye");
    }
}

// 创建接收者结构体
struct Receiver {
    commands: Vec<Box<dyn Command>>,
}

impl Receiver {
    fn new() -> Self {
        Receiver {
            commands: Vec::new(),
        }
    }

    fn add_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    fn execute_commands(&self) {
        for command in &self.commands {
            command.execute();
        }
    }
}

fn main() {
    let mut receiver = Receiver::new();
    receiver.add_command(Box::new(PrintHello));
    receiver.add_command(Box::new(PrintGoodbye));
    receiver.execute_commands();
}
