// 状态接口，定义了所有具体状态必须实现的方法
interface State {
    handle(context: Context): void;
}

// 上下文类，持有状态的引用并允许状态改变
class Context {
    private state: State;

    constructor(state: State) {
        this.state = state;
    }

    public setState(state: State): void {
        this.state = state;
    }

    public request(): void {
        this.state.handle(this);
    }
}

// 开始状态的具体实现
class StartState implements State {
    public handle(context: Context): void {
        console.log('Player is in start state');
        context.setState(new StopState());
    }
}

// 停止状态的具体实现
class StopState implements State {
    public handle(context: Context): void {
        console.log('Player is in stop state');
        // 可能会设置其他状态，但在这个简单的例子中，我们不做改变
    }
}

// 实例化上下文对象，初始状态设置为 StartState
let context = new Context(new StartState());

// 第一次请求会打印出 "Player is in start state"
// 并将状态改变为 StopState
context.request();

// 第二次请求会打印出 "Player is in stop state"
context.request();

// 第三次请求仍然是 StopState，输出不会改变
context.request();
