trait EnemyBehavior {
    // 这些方法可以被具体类型重写
    fn search_player(&self);
    fn attack(&self);
    fn flee(&self);

    // 模板方法，定义行为的顺序
    fn act(&self) {
        self.search_player();
        self.attack();
        self.flee();
    }
}

struct Goblin;

impl EnemyBehavior for Goblin {
    fn search_player(&self) {
        println!("Goblin is searching for the player...");
    }

    fn attack(&self) {
        println!("Goblin attacks!");
    }

    fn flee(&self) {
        println!("Goblin flees after attacking.");
    }
}

struct Orc;

impl EnemyBehavior for Orc {
    fn search_player(&self) {
        println!("Orc is looking around for the player...");
    }

    fn attack(&self) {
        println!("Orc charges with a roar!");
    }

    fn flee(&self) {
        println!("Orc doesn't flee, it fights till the end!");
    }
}

fn main() {
    let goblin = Goblin;
    let orc = Orc;

    println!("Goblin's action:");
    goblin.act();

    println!("\nOrc's action:");
    orc.act();
}
