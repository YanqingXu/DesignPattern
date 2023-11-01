// 表达式接口
interface Expression {
    interpret(context: InterpreterContext): number;
}

// 数字表达式
class NumberExpression implements Expression {
    private value: number;

    constructor(value: string) {
        this.value = parseInt(value);
    }

    interpret(context: InterpreterContext): number {
        return this.value;
    }
}

// 加法表达式
class AddExpression implements Expression {
    private leftExpression: Expression;
    private rightExpression: Expression;

    constructor(left: Expression, right: Expression) {
        this.leftExpression = left;
        this.rightExpression = right;
    }

    interpret(context: InterpreterContext): number {
        return this.leftExpression.interpret(context) + this.rightExpression.interpret(context);
    }
}

// 减法表达式
class SubtractExpression implements Expression {
    private leftExpression: Expression;
    private rightExpression: Expression;

    constructor(left: Expression, right: Expression) {
        this.leftExpression = left;
        this.rightExpression = right;
    }

    interpret(context: InterpreterContext): number {
        return this.leftExpression.interpret(context) - this.rightExpression.interpret(context);
    }
}

// 变量表达式
class VariableExpression implements Expression {
    private name: string;

    constructor(name: string) {
        this.name = name;
    }

    interpret(context: InterpreterContext): number {
        return context.lookup(this.name);
    }

    getName(): string {
        return this.name;
    }
}

// 解释器上下文
class InterpreterContext {
    private variables: Map<string, number>;

    constructor() {
        this.variables = new Map<string, number>();
    }

    assign(variable: VariableExpression, value: number) {
        this.variables.set(variable.getName(), value);
    }

    lookup(name: string): number {
        if (!this.variables.has(name)) {
            throw new Error(`Undefined variable ${name}`);
        }
        return this.variables.get(name)!;
    }
}

// 客户端代码
function main() {
    const context = new InterpreterContext();

    // 定义一些变量
    const x = new VariableExpression("x");
    const y = new VariableExpression("y");

    // 为变量赋值
    context.assign(x, 10);
    context.assign(y, 20);

    // 构造一个简单的表达式：x + y - 5
    const expression = new SubtractExpression(
        new AddExpression(x, y),
        new NumberExpression("5")
    );

    // 解释表达式
    const result = expression.interpret(context);

    console.log(result); // 输出结果应为25
}

main();
