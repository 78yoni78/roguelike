pub use tcod::input::Key;
pub use tcod::input::KeyCode;

pub trait InputHandler {
    fn wait_for_keypress(&mut self) -> Key;
} 