trait Character {
    fn describe(&self) -> &'static str;
}

trait Enemy {
    fn describe(&self) -> &'static str;
}

trait GameFactory {
    fn create_character(&self) -> Box<dyn Character>;
    fn create_enemy(&self) -> Box<dyn Enemy>;
}

// 战士
struct Warrior;

impl Character for Warrior {
    fn describe(&self) -> &'static str {
        "Warrior"
    }
}

// 法师
struct Mage;

impl Character for Mage {
    fn describe(&self) -> &'static str {
        "Mage"
    }
}

// 兽人
struct Orc;

impl Enemy for Orc {
    fn describe(&self) -> &'static str {
        "Orc"
    }
}

// 哥布林
struct Goblin;

impl Enemy for Goblin {
    fn describe(&self) -> &'static str {
        "Goblin"
    }
}

// 中世纪工厂
struct MedievalFactory;

impl GameFactory for MedievalFactory {
    // 中世纪工厂生产战士
    fn create_character(&self) -> Box<dyn Character> {
        Box::new(Warrior)
    }

    // 中世纪工厂生产兽人
    fn create_enemy(&self) -> Box<dyn Enemy> {
        Box::new(Orc)
    }
}

// 奇幻工厂
struct FantasyFactory;

impl GameFactory for FantasyFactory {
    // 奇幻工厂生产法师
    fn create_character(&self) -> Box<dyn Character> {
        Box::new(Mage)
    }

    // 奇幻工厂生产哥布林
    fn create_enemy(&self) -> Box<dyn Enemy> {
        Box::new(Goblin)
    }
}

fn main() {
    let medieval_factory: Box<dyn GameFactory> = Box::new(MedievalFactory);
    let character = medieval_factory.create_character();
    let enemy = medieval_factory.create_enemy();
    println!("Character: {}", character.describe());
    println!("Enemy: {}", enemy.describe());

    let fantasy_factory: Box<dyn GameFactory> = Box::new(FantasyFactory);
    let character = fantasy_factory.create_character();
    let enemy = fantasy_factory.create_enemy();
    println!("Character: {}", character.describe());
    println!("Enemy: {}", enemy.describe());
}
