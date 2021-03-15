pub type Key = tcod::input::Key;
pub type KeyCode = tcod::input::KeyCode;

pub trait InputHandler {
    fn wait_for_keypress(&mut self) -> Key;
} 