// 元素接口
interface Element {
    accept(visitor: Visitor): void;
}

// 角色类
class Character implements Element {
    accept(visitor: Visitor): void {
        visitor.visitCharacter(this);
    }

    // 角色特定的方法
    getCharacterDetails(): string {
        return 'Character details...';
    }
}

// 敌人类
class Enemy implements Element {
    accept(visitor: Visitor): void {
        visitor.visitEnemy(this);
    }

    // 敌人特定的方法
    getEnemyDetails(): string {
        return 'Enemy details...';
    }
}

// 道具类
class Item implements Element {
    accept(visitor: Visitor): void {
        visitor.visitItem(this);
    }

    // 道具特定的方法
    getItemDetails(): string {
        return 'Item details...';
    }
}

// 访问者接口
interface Visitor {
    visitCharacter(character: Character): void;
    visitEnemy(enemy: Enemy): void;
    visitItem(item: Item): void;
}

// 调试和测试访问者
class DebugVisitor implements Visitor {
    visitCharacter(character: Character): void {
        console.log('Debugging Character: ' + character.getCharacterDetails());
    }

    visitEnemy(enemy: Enemy): void {
        console.log('Debugging Enemy: ' + enemy.getEnemyDetails());
    }

    visitItem(item: Item): void {
        console.log('Debugging Item: ' + item.getItemDetails());
    }
}

function main(): void {
    const elements: Element[] = [new Character(), new Enemy(), new Item()];

    const debugVisitor = new DebugVisitor();

    elements.forEach(element => {
        element.accept(debugVisitor);
    });
}

main();
