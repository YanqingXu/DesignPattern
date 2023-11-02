class Memento {
    private state: string;

    constructor(state: string) {
        this.state = state;
    }

    public getState(): string {
        return this.state;
    }
}

class Originator {
    private state: string = ''; // 提供一个默认值

    public setState(state: string): void {
        this.state = state;
        console.log(`State is set to: ${this.state}`);
    }

    public getState(): string {
        return this.state;
    }

    public save(): Memento {
        console.log('Saving state.');
        return new Memento(this.state);
    }

    public restore(memento: Memento): void {
        this.state = memento.getState();
        console.log(`State after restoring: ${this.state}`);
    }
}

class Caretaker {
    private mementos: Memento[] = [];

    public addMemento(memento: Memento): void {
        this.mementos.push(memento);
    }

    public getMemento(index: number): Memento {
        return this.mementos[index];
    }
}

// Usage
const originator = new Originator();
const caretaker = new Caretaker();

originator.setState('State #1');
caretaker.addMemento(originator.save());

originator.setState('State #2');
caretaker.addMemento(originator.save());

originator.setState('State #3');
console.log(`Current State: ${originator.getState()}`);

// Restore to State #2
originator.restore(caretaker.getMemento(1));
console.log(`Current State: ${originator.getState()}`);

// Restore to State #1
originator.restore(caretaker.getMemento(0));
console.log(`Current State: ${originator.getState()}`);
