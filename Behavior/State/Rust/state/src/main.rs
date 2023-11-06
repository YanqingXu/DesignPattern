pub trait State {
    fn handle(self: Box<Self>, context: &mut Context);
}

pub struct Context {
    state: Option<Box<dyn State>>,
}

impl Context {
    fn new(initial_state: Box<dyn State>) -> Context {
        Context {
            state: Some(initial_state),
        }
    }

    fn request(&mut self) {
        if let Some(state) = self.state.take() {
            state.handle(self);
        }
    }

    fn change_state(&mut self, new_state: Box<dyn State>) {
        self.state = Some(new_state);
    }
}

struct StartState;
struct StopState;

impl State for StartState {
    fn handle(self: Box<Self>, context: &mut Context) {
        println!("Player is in start state");
        context.change_state(Box::new(StopState));
    }
}

impl State for StopState {
    fn handle(self: Box<Self>, context: &mut Context) {
        println!("Player is in stop state");
    }
}

fn main() {
    let initial_state = Box::new(StartState);
    let mut context = Context::new(initial_state);

    // The state should be StartState here
    context.request();

    // The state should now be StopState
    context.request();

    // The state is still StopState
    context.request();
}
