use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 观察者特性
trait Observer {
    fn update(&self, temperature: f32);
}

// 被观察者特性
trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: &Weak<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

// 天气数据结构
struct WeatherData {
    temperature: f32,
    observers: Vec<Weak<RefCell<dyn Observer>>>,
}

impl WeatherData {
    fn new() -> WeatherData {
        WeatherData {
            temperature: 0.0,
            observers: Vec::new(),
        }
    }

    fn set_temperature(&mut self, temperature: f32) {
        self.temperature = temperature;
        self.notify_observers();
    }
}

impl Subject for WeatherData {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(Rc::downgrade(&observer));
    }

    fn remove_observer(&mut self, observer: &Weak<RefCell<dyn Observer>>) {
        self.observers.retain(|obs| !Weak::ptr_eq(obs, observer));
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            if let Some(obs) = observer.upgrade() {
                obs.borrow().update(self.temperature);
            }
        }
    }
}

// 显示板结构
struct Display {}

impl Observer for Display {
    fn update(&self, temperature: f32) {
        println!("Current temperature is: {}°C", temperature);
    }
}

impl Display {
    fn new() -> Display {
        Display {}
    }
}

fn main() {
    let weather_data = Rc::new(RefCell::new(WeatherData::new()));
    let display = Rc::new(RefCell::new(Display::new()));

    {
        let mut wd = weather_data.borrow_mut();
        wd.register_observer(display.clone());
        wd.set_temperature(23.0);
    }

    // Current temperature is: 23°C

    // 更改天气数据并通知观察者
    {
        let mut wd = weather_data.borrow_mut();
        wd.set_temperature(26.5);
    }

    // Current temperature is: 26.5°C

    // 移除观察者
    {
        let mut wd = weather_data.borrow_mut();
        let weak_display: Weak<RefCell<dyn Observer>> =
            Rc::downgrade(&display) as Weak<RefCell<dyn Observer>>;
        wd.remove_observer(&weak_display);
        wd.set_temperature(30.0);
    }

    // 此时将不会有输出，因为观察者已经被移除了
}
