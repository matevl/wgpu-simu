use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default, Debug)]
pub struct InputState {
    pressed_keys: HashSet<KeyCode>,
    mouse_position: (f64, f64),
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_keyboard_input(&mut self, key: KeyCode, is_pressed: bool) {
        if is_pressed {
            self.pressed_keys.insert(key);
        } else {
            self.pressed_keys.remove(&key);
        }
    }

    pub fn on_cursor_moved(&mut self, x: f64, y: f64) {
        self.mouse_position = (x, y);
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn cursor_pos(&self) -> (f64, f64) {
        self.mouse_position
    }
}
