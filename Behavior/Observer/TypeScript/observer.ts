// Observer.ts
interface Observer {
    update(event: GameEvent): void;
}

// Subject.ts
interface Subject {
    registerObserver(observer: Observer): void;
    removeObserver(observer: Observer): void;
    notifyObservers(event: GameEvent): void; // Add GameEvent parameter here
}

// GameEvent.ts
enum GameEventType {
    PlayerHealthChanged,
    EnemyDefeated,
    // ...更多游戏事件
}

class GameEvent {
    constructor(public type: GameEventType, public payload?: any) { }
}

// GameSubject.ts
class GameSubject implements Subject {
    private observers: Observer[] = [];

    registerObserver(observer: Observer): void {
        const isExist = this.observers.includes(observer);
        if (isExist) {
            return console.log('Observer has been registered already.');
        }
        this.observers.push(observer);
    }

    removeObserver(observer: Observer): void {
        const observerIndex = this.observers.indexOf(observer);
        if (observerIndex === -1) {
            return console.log('Nonexistent observer.');
        }
        this.observers.splice(observerIndex, 1);
    }

    notifyObservers(event: GameEvent): void {
        for (const observer of this.observers) {
            observer.update(event);
        }
    }
}

// ConcreteObservers.ts
class PlayerComponent implements Observer {
    update(event: GameEvent): void {
        if (event.type === GameEventType.PlayerHealthChanged) {
            console.log('Player health changed:', event.payload);
        }
    }
}

class EnemyComponent implements Observer {
    update(event: GameEvent): void {
        if (event.type === GameEventType.EnemyDefeated) {
            console.log('An enemy has been defeated.');
        }
    }
}


// Demo.ts
const gameSubject = new GameSubject();

const player = new PlayerComponent();
const enemy = new EnemyComponent();

gameSubject.registerObserver(player);
gameSubject.registerObserver(enemy);

// 模拟玩家生命值变化事件
gameSubject.notifyObservers(new GameEvent(GameEventType.PlayerHealthChanged, { newHealth: 75 }));

// 模拟敌人被击败事件
gameSubject.notifyObservers(new GameEvent(GameEventType.EnemyDefeated));
