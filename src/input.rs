pub use tcod::input::{Key, KeyCode};

pub trait InputHandler {
    fn wait_for_keypress(&mut self) -> Key;
} 