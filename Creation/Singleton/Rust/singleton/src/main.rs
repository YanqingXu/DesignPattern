use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, Once};

pub struct Singleton<T>
where
    T: Default + 'static,
{
    instance: Mutex<Option<Arc<T>>>,
    once: Once,
}

impl<T> Singleton<T>
where
    T: Default + 'static,
{
    pub fn new() -> Self {
        Singleton {
            instance: Mutex::new(None),
            once: Once::new(),
        }
    }

    pub fn get_instance(&self) -> Arc<T> {
        self.once.call_once(|| {
            let instance = T::default();
            *self.instance.lock().unwrap() = Some(Arc::new(instance));
        });
        self.instance.lock().unwrap().as_ref().unwrap().clone()
    }
}

lazy_static! {
    static ref INT_HOLDER: Singleton<i32> = Singleton::new();
    static ref STRING_HOLDER: Singleton<String> = Singleton::new();
}

fn main() {
    let int_instance1 = INT_HOLDER.get_instance();
    let int_instance2 = INT_HOLDER.get_instance();

    if Arc::ptr_eq(&int_instance1, &int_instance2) {
        println!("两个i32实例是相同的");
    } else {
        println!("两个i32实例是不同的");
    }

    let string_instance1 = STRING_HOLDER.get_instance();
    let string_instance2 = STRING_HOLDER.get_instance();

    if Arc::ptr_eq(&string_instance1, &string_instance2) {
        println!("两个String实例是相同的");
    } else {
        println!("两个String实例是不同的");
    }
}

/* 我们使用了泛型 T 来定义 Singleton 结构体。这样我们就可以为任何实现了 Default trait 的类型创建单例。
在 main 函数中，我们为 i32 和 String 类型创建了单例来演示如何使用这个泛型单例。 */
