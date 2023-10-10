// 定义道具结构
struct Item {
    name: String,
    effect: String,
}

// 定义道具效果处理器特征
trait ItemEffectHandler {
    fn handle_effect(&self, item: &mut Item);
    fn set_next_handler(&mut self, next: Box<dyn ItemEffectHandler>);
}

// 具体的道具效果处理器
struct HealthPotionHandler;
impl ItemEffectHandler for HealthPotionHandler {
    fn handle_effect(&self, item: &mut Item) {
        if item.effect == "Health" {
            println!("Applying Health Potion effect to {}", item.name);
            // 处理道具效果逻辑
        }
    }

    fn set_next_handler(&mut self, _next: Box<dyn ItemEffectHandler>) {
        // HealthPotionHandler 是责任链中的最后一个处理器，不设置下一个处理器
    }
}

struct ManaPotionHandler;
impl ItemEffectHandler for ManaPotionHandler {
    fn handle_effect(&self, item: &mut Item) {
        if item.effect == "Mana" {
            println!("Applying Mana Potion effect to {}", item.name);
            // 处理道具效果逻辑
        }
    }

    fn set_next_handler(&mut self, next: Box<dyn ItemEffectHandler>) {
        // ManaPotionHandler 是责任链中的最后一个处理器，不设置下一个处理器
    }
}

// 游戏中的道具处理器链
struct ItemEffectChain {
    handler: Option<Box<dyn ItemEffectHandler>>,
}

impl ItemEffectChain {
    fn new() -> ItemEffectChain {
        ItemEffectChain { handler: None }
    }

    fn add_handler(&mut self, handler: Box<dyn ItemEffectHandler>) {
        match &mut self.handler {
            Some(existing_handler) => existing_handler.set_next_handler(handler),
            None => self.handler = Some(handler),
        }
    }

    fn process_item(&self, mut item: Item) {
        match &self.handler {
            Some(handler) => handler.handle_effect(&mut item),
            None => println!("No handler available for {}", item.name),
        }
    }
}

fn main() {
    // 创建道具效果处理器链
    let mut effect_chain = ItemEffectChain::new();

    // 添加具体的道具效果处理器
    effect_chain.add_handler(Box::new(HealthPotionHandler));
    effect_chain.add_handler(Box::new(ManaPotionHandler));

    // 模拟道具效果处理
    let item1 = Item {
        name: "Health Potion".to_string(),
        effect: "Health".to_string(),
    };
    effect_chain.process_item(item1);

    let item2 = Item {
        name: "Mana Potion".to_string(),
        effect: "Mana".to_string(),
    };
    effect_chain.process_item(item2);

    let item3 = Item {
        name: "Strength Potion".to_string(),
        effect: "Strength".to_string(),
    };
    effect_chain.process_item(item3);
}
