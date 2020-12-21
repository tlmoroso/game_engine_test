use std::collections::HashSet;
use coffee::input::keyboard::KeyCode;
use coffee::input::{Input, keyboard};
use coffee::input;

#[derive(Default, Debug)]
pub struct TestCustomInput {
    pub keys_pressed: HashSet<KeyCode>,
    pub keys_held: HashSet<KeyCode>,
    pub keys_released: HashSet<KeyCode>,
}

impl TestCustomInput {
    pub fn get_keys_pressed(&mut self) -> &HashSet<KeyCode> {
        &self.keys_pressed
    }

    pub fn get_keys_held(&mut self) -> &HashSet<KeyCode> {
        &self.keys_held
    }

    pub fn get_keys_released(&mut self) -> &HashSet<KeyCode> {
        &self.keys_released
    }
}

impl Input for TestCustomInput {
    fn new() -> TestCustomInput {
        TestCustomInput {
            keys_pressed: HashSet::new(),
            keys_held: HashSet::new(),
            keys_released: HashSet::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        match event {
            input::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::Input { key_code, state } => match state {
                    input::ButtonState::Pressed => {
                        if !self.keys_held.contains(&key_code) {
                            self.keys_pressed.insert(key_code);
                        }
                    },
                    input::ButtonState::Released => {
                        self.keys_released.insert(key_code);
                        self.keys_held.remove(&key_code);
                    }
                },
                _ => (),
            },
            _ => (),
        }
    }

    fn clear(&mut self) {
        let new_keys_held = self.keys_pressed.clone();
        for key in new_keys_held {
            self.keys_held.insert(key);
        }

        self.keys_pressed.clear();
        self.keys_released.clear();
    }
}