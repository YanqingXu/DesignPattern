trait DifficultyStrategy {
    fn adjust_difficulty(&self);
}

struct EasyDifficulty;

impl DifficultyStrategy for EasyDifficulty {
    fn adjust_difficulty(&self) {
        println!("Setting game difficulty to Easy.");
        // 实现易难度的具体逻辑
    }
}

struct MediumDifficulty;

impl DifficultyStrategy for MediumDifficulty {
    fn adjust_difficulty(&self) {
        println!("Setting game difficulty to Medium.");
        // 实现中等难度的具体逻辑
    }
}

struct HardDifficulty;

impl DifficultyStrategy for HardDifficulty {
    fn adjust_difficulty(&self) {
        println!("Setting game difficulty to Hard.");
        // 实现难难度的具体逻辑
    }
}

struct GameContext {
    difficulty: Box<dyn DifficultyStrategy>,
}

impl GameContext {
    fn new(difficulty: Box<dyn DifficultyStrategy>) -> GameContext {
        GameContext { difficulty }
    }

    fn set_difficulty(&mut self, difficulty: Box<dyn DifficultyStrategy>) {
        self.difficulty = difficulty;
    }

    fn play(&self) {
        self.difficulty.adjust_difficulty();
        // 游戏的其他逻辑
    }
}

fn main() {
    let mut game = GameContext::new(Box::new(EasyDifficulty));
    game.play(); // 输出: Setting game difficulty to Easy.

    // 改变游戏难度为中等
    game.set_difficulty(Box::new(MediumDifficulty));
    game.play(); // 输出: Setting game difficulty to Medium.

    // 改变游戏难度为困难
    game.set_difficulty(Box::new(HardDifficulty));
    game.play(); // 输出: Setting game difficulty to Hard.
}
