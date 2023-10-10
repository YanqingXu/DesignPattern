abstract class Handler {
    protected nextHandler: Handler | null = null;

    public setNext(handler: Handler): Handler {
        this.nextHandler = handler;
        return handler;
    }

    public handle(request: string): string {
        if (this.nextHandler) {
            return this.nextHandler.handle(request);
        }

        return '';
    }
}

class ConcreteHandler1 extends Handler {
    public handle(request: string): string {
        if (request === 'request1') {
            return 'ConcreteHandler1 handled the request';
        } else if (this.nextHandler) {
            return this.nextHandler.handle(request);
        }

        return '';
    }
}

class ConcreteHandler2 extends Handler {
    public handle(request: string): string {
        if (request === 'request2') {
            return 'ConcreteHandler2 handled the request';
        } else if (this.nextHandler) {
            return this.nextHandler.handle(request);
        }

        return '';
    }
}

// 使用示例
const handler1 = new ConcreteHandler1();
const handler2 = new ConcreteHandler2();

handler1.setNext(handler2);

console.log(handler1.handle('request1'));  // Output: ConcreteHandler1 handled the request
console.log(handler1.handle('request2'));  // Output: ConcreteHandler2 handled the request
