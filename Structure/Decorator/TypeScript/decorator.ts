// 基本角色类
class Character {
    constructor(public name: string, public baseAttack: number, public baseDefense: number) { }

    getInfo() {
        console.log(`Name: ${this.name}, Attack: ${this.baseAttack}, Defense: ${this.baseDefense}`);
    }
}

// 装饰器接口
interface CharacterDecorator {
    decorate(character: Character): Character;
}

// 具体装饰器 - 增加攻击力
class AttackDecorator implements CharacterDecorator {
    constructor(private attackBonus: number) { }

    decorate(character: Character): Character {
        character.baseAttack += this.attackBonus;
        return character;
    }
}

// 具体装饰器 - 增加防御力
class DefenseDecorator implements CharacterDecorator {
    constructor(private defenseBonus: number) { }

    decorate(character: Character): Character {
        character.baseDefense += this.defenseBonus;
        return character;
    }
}

// 创建基本角色
const player = new Character("Player", 10, 5);
player.getInfo();

// 使用装饰器增加属性
const playerWithAttackBonus = new AttackDecorator(5).decorate(player);
playerWithAttackBonus.getInfo();

const playerWithDefenseBonus = new DefenseDecorator(3).decorate(player);
playerWithDefenseBonus.getInfo();
