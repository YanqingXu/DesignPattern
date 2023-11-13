// 元素特征
pub trait Element {
    fn accept(&self, visitor: &mut dyn Visitor);
}

// 角色
pub struct Character;
impl Element for Character {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_character(self);
    }
}

// 敌人
pub struct Enemy;
impl Element for Enemy {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_enemy(self);
    }
}

// 道具
pub struct Item;
impl Element for Item {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_item(self);
    }
}

// 访问者特征
pub trait Visitor {
    fn visit_character(&mut self, _character: &Character);
    fn visit_enemy(&mut self, _enemy: &Enemy);
    fn visit_item(&mut self, _item: &Item);
}

// 音效和动画访问者
pub struct AudioAnimationVisitor;
impl Visitor for AudioAnimationVisitor {
    fn visit_character(&mut self, _character: &Character) {
        println!("Playing character animation and sound");
        // 实现音效和动画处理逻辑
    }

    fn visit_enemy(&mut self, _enemy: &Enemy) {
        println!("Playing enemy animation and sound");
        // 实现音效和动画处理逻辑
    }

    fn visit_item(&mut self, _item: &Item) {
        println!("Playing item animation and sound");
        // 实现音效和动画处理逻辑
    }
}

fn main() {
    let elements: Vec<Box<dyn Element>> =
        vec![Box::new(Character), Box::new(Enemy), Box::new(Item)];

    let mut visitor = AudioAnimationVisitor;

    for element in elements.iter() {
        element.accept(&mut visitor);
    }
}
