interface DifficultyStrategy {
    adjustDifficulty(): void;
}

class EasyDifficulty implements DifficultyStrategy {
    adjustDifficulty(): void {
        console.log("Setting game difficulty to Easy.");
        // 实现易难度的具体逻辑
    }
}

class MediumDifficulty implements DifficultyStrategy {
    adjustDifficulty(): void {
        console.log("Setting game difficulty to Medium.");
        // 实现中等难度的具体逻辑
    }
}

class HardDifficulty implements DifficultyStrategy {
    adjustDifficulty(): void {
        console.log("Setting game difficulty to Hard.");
        // 实现难难度的具体逻辑
    }
}

class GameContext {
    private difficulty: DifficultyStrategy;

    constructor(difficulty: DifficultyStrategy) {
        this.difficulty = difficulty;
    }

    setDifficulty(difficulty: DifficultyStrategy): void {
        this.difficulty = difficulty;
    }

    startGame(): void {
        this.difficulty.adjustDifficulty();
        // 游戏的其他逻辑
    }
}

let game = new GameContext(new EasyDifficulty());
game.startGame();  // 输出: Setting game difficulty to Easy.

// 改变游戏难度为中等
game.setDifficulty(new MediumDifficulty());
game.startGame();  // 输出: Setting game difficulty to Medium.

// 改变游戏难度为困难
game.setDifficulty(new HardDifficulty());
game.startGame();  // 输出: Setting game difficulty to Hard.
