// Command 接口定义命令的执行操作
interface Command {
    execute(): void;
}

// Character 类表示游戏中的角色
class Character {
    private position: number = 0;

    move(distance: number): void {
        this.position += distance;
        console.log(`Moved ${distance} units. Current position: ${this.position}`);
    }
}

// MoveCommand 表示移动命令
class MoveCommand implements Command {
    private character: Character;
    private distance: number;

    constructor(character: Character, distance: number) {
        this.character = character;
        this.distance = distance;
    }

    execute(): void {
        this.character.move(this.distance);
    }
}

// CommandQueue 类用于管理命令队列
class CommandQueue {
    private commands: Command[] = [];
    private currentIndex: number = 0;

    addCommand(command: Command): void {
        this.commands.push(command);
    }

    getNextCommand(): Command | null {
        if (this.currentIndex < this.commands.length) {
            return this.commands[this.currentIndex++];
        }
        return null;
    }
}

// 创建游戏角色
const player = new Character();

// 创建命令队列
const commandQueue = new CommandQueue();

// 创建移动命令
const moveCommand1 = new MoveCommand(player, 5);
const moveCommand2 = new MoveCommand(player, -3);

// 添加命令到队列
commandQueue.addCommand(moveCommand1);
commandQueue.addCommand(moveCommand2);

// 执行队列中的命令
let command = commandQueue.getNextCommand();
while (command) {
    command.execute();
    command = commandQueue.getNextCommand();
}
