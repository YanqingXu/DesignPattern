// Mediator interface
interface Mediator {
    send(message: string, colleague: Colleague): void;
}

// Colleague interface
interface Colleague {
    setMediator(mediator: Mediator): void;
    send(message: string): void;
    receive(message: string): void;
}


// ConcreteMediator class
class ConcreteMediator implements Mediator {
    private colleagues: Colleague[] = [];

    public register(colleague: Colleague): void {
        this.colleagues.push(colleague);
        colleague.setMediator(this);
    }

    public send(message: string, sender: Colleague): void {
        for (const colleague of this.colleagues) {
            // The sender doesn't receive its own message
            if (colleague !== sender) {
                colleague.receive(message);
            }
        }
    }
}

// ConcreteColleague class
class ConcreteColleague implements Colleague {
    private mediator?: Mediator;

    constructor(private readonly id: number) { }

    public setMediator(mediator: Mediator): void {
        this.mediator = mediator;
    }

    public send(message: string): void {
        if (this.mediator) {
            console.log(`Colleague ${this.id} sends message: ${message}`);
            this.mediator.send(message, this);
        }
    }

    public receive(message: string): void {
        console.log(`Colleague ${this.id} received message: ${message}`);
    }
}

// Usage
const mediator = new ConcreteMediator();

const colleague1 = new ConcreteColleague(1);
const colleague2 = new ConcreteColleague(2);
const colleague3 = new ConcreteColleague(3);

mediator.register(colleague1);
mediator.register(colleague2);
mediator.register(colleague3);

colleague1.send("Hello World");
colleague2.send("Hello Moon");
colleague3.send("Hello Sun");
