trait Expression {
    fn interpret(&self, context: &mut Context) -> i32;
}

struct Context {
    input: String,
    output: i32,
}

struct NumberExpression {
    number: i32,
}

impl NumberExpression {
    fn new(number: i32) -> Self {
        NumberExpression { number }
    }
}

impl Expression for NumberExpression {
    fn interpret(&self, _context: &mut Context) -> i32 {
        self.number
    }
}

struct AddExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl AddExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        AddExpression { left, right }
    }
}

impl Expression for AddExpression {
    fn interpret(&self, context: &mut Context) -> i32 {
        self.left.interpret(context) + self.right.interpret(context)
    }
}

struct SubtractExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl SubtractExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        SubtractExpression { left, right }
    }
}

impl Expression for SubtractExpression {
    fn interpret(&self, context: &mut Context) -> i32 {
        self.left.interpret(context) - self.right.interpret(context)
    }
}

// 用于解析输入并产生相应的表达式树
fn parse(input: &str) -> Box<dyn Expression> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut stack: Vec<Box<dyn Expression>> = Vec::new();

    for i in 0..tokens.len() {
        match tokens[i] {
            "+" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Box::new(AddExpression::new(left, right)));
            }
            "-" => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(Box::new(SubtractExpression::new(left, right)));
            }
            _ => {
                stack.push(Box::new(NumberExpression::new(tokens[i].parse().unwrap())));
            }
        }
    }

    stack.pop().unwrap()
}

fn main() {
    let input = "3 + 4 - 2";
    let mut context = Context {
        input: input.to_string(),
        output: 0,
    };
    let expression = parse(input);
    let result = expression.interpret(&mut context);
    println!("Result: {}", result);
}
