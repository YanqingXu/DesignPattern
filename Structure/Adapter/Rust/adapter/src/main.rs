// Target interface
trait InputDevice {
    fn get_input(&self) -> String;
}

// Adaptee 1: Keyboard
struct Keyboard;

impl Keyboard {
    fn read_key(&self) -> String {
        "Key pressed".to_string()
    }
}

// Adapter for Keyboard
struct KeyboardInputAdapter {
    device: Keyboard,
}

impl InputDevice for KeyboardInputAdapter {
    fn get_input(&self) -> String {
        self.device.read_key()
    }
}

// Adaptee 2: Gamepad
struct Gamepad;

impl Gamepad {
    fn read_button(&self) -> String {
        "Button pressed".to_string()
    }
}

// Adapter for Gamepad
struct GamepadInputAdapter {
    device: Gamepad,
}

impl InputDevice for GamepadInputAdapter {
    fn get_input(&self) -> String {
        self.device.read_button()
    }
}

// Function to process input, regardless of the device
fn process_input(device: &dyn InputDevice) {
    println!("{}", device.get_input());
}

fn main() {
    let keyboard = KeyboardInputAdapter {
        device: Keyboard {},
    };
    let gamepad = GamepadInputAdapter { device: Gamepad {} };

    process_input(&keyboard);
    process_input(&gamepad);
}
