struct Memento {
    state: String,
}

impl Memento {
    fn new(state: String) -> Memento {
        Memento { state }
    }

    fn get_state(&self) -> &String {
        &self.state
    }
}

struct Originator {
    state: String,
}

impl Originator {
    fn new() -> Originator {
        Originator {
            state: String::new(),
        }
    }

    fn set_state(&mut self, state: String) {
        self.state = state;
    }

    fn save_to_memento(&self) -> Memento {
        Memento::new(self.state.clone())
    }

    fn restore_from_memento(&mut self, memento: &Memento) {
        self.state = memento.get_state().clone();
    }
}

struct Caretaker {
    saved_states: Vec<Memento>,
}

impl Caretaker {
    fn new() -> Caretaker {
        Caretaker {
            saved_states: Vec::new(),
        }
    }

    fn add_memento(&mut self, memento: Memento) {
        self.saved_states.push(memento);
    }

    fn get_memento(&self, index: usize) -> Option<&Memento> {
        self.saved_states.get(index)
    }
}

fn main() {
    let mut originator = Originator::new();
    let mut caretaker = Caretaker::new();

    originator.set_state("State1".to_string());
    caretaker.add_memento(originator.save_to_memento());
    originator.set_state("State2".to_string());
    caretaker.add_memento(originator.save_to_memento());

    originator.restore_from_memento(caretaker.get_memento(0).unwrap());
    println!("Restored to: {}", originator.state); // Should print "Restored to: State1"
}
