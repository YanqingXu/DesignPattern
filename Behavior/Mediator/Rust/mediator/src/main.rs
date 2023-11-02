use std::cell::RefCell;
use std::rc::Rc;

trait Mediator {
    fn send(&self, message: &str, colleague_id: usize);
}

trait Colleague {
    fn receive(&self, message: &str);
    fn send(&self, message: &str);
    fn set_mediator(&mut self, mediator: Rc<RefCell<dyn Mediator>>);
    fn get_id(&self) -> usize;
}

struct ConcreteMediator {
    colleagues: Vec<Rc<RefCell<dyn Colleague>>>,
}

impl ConcreteMediator {
    fn new() -> Self {
        ConcreteMediator {
            colleagues: Vec::new(),
        }
    }

    fn add_colleague(&mut self, colleague: Rc<RefCell<dyn Colleague>>) {
        self.colleagues.push(colleague);
    }
}

impl Mediator for ConcreteMediator {
    fn send(&self, message: &str, colleague_id: usize) {
        for colleague in &self.colleagues {
            let c = colleague.borrow();
            if c.get_id() != colleague_id {
                c.receive(message);
            }
        }
    }
}

struct ConcreteColleague {
    id: usize,
    mediator: Option<Rc<RefCell<dyn Mediator>>>,
}

impl ConcreteColleague {
    fn new(id: usize) -> Self {
        ConcreteColleague { id, mediator: None }
    }
}

impl Colleague for ConcreteColleague {
    fn receive(&self, message: &str) {
        println!("Colleague {} received: {}", self.id, message);
    }

    fn send(&self, message: &str) {
        if let Some(ref mediator) = self.mediator {
            mediator.borrow().send(message, self.id);
        }
    }

    fn set_mediator(&mut self, mediator: Rc<RefCell<dyn Mediator>>) {
        self.mediator = Some(mediator);
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

fn main() {
    let mediator = Rc::new(RefCell::new(ConcreteMediator::new()));

    let colleague1 = Rc::new(RefCell::new(ConcreteColleague::new(1)));
    colleague1.borrow_mut().set_mediator(mediator.clone());

    let colleague2 = Rc::new(RefCell::new(ConcreteColleague::new(2)));
    colleague2.borrow_mut().set_mediator(mediator.clone());

    mediator.borrow_mut().add_colleague(colleague1.clone());
    mediator.borrow_mut().add_colleague(colleague2.clone());

    colleague1.borrow().send("Hello from colleague 1");
    colleague2.borrow().send("Hello from colleague 2");
}
