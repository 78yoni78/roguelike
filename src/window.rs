use tcod::console::{Root, Offscreen, Console};

use crate::input::{InputHandler, Key};

#[derive(Debug)]
pub struct Config {
    pub title: String,
    pub fps_limit: i32,
    pub size: (u16, u16),
}

pub struct Window {
    config: Config,
    root: Root,
    game_screen: Offscreen,
}

impl Window {
    pub fn new(config: Config) -> Self {
        let root = Root::initializer()
        .font("arial10x10.png", tcod::FontLayout::Tcod)
        .font_type(tcod::FontType::Greyscale)
        .size(config.size.0 as i32, config.size.1 as i32)
        .title(&config.title)
        .init();
        
        let game_screen = Offscreen::new(config.size.0 as i32, config.size.1 as i32);

        tcod::system::set_fps(config.fps_limit);
    
        Window { config, root, game_screen }
    }

    pub fn closed(&self) -> bool {
        self.root.window_closed()
    }

    pub fn redraw<F: FnMut(dyn Console)>(&mut self, func: F) {
        use std::cmp::min;

        //  Clear the offscreen
        self.game_screen.clear();
        func(&mut self.game_screen);
        //  Draw the offscreen onto the root screen and flush
        tcod::console::blit(
            &self.game_screen,
            (0, 0),
            (min(self.game_screen.width(), self.size.0), min(self.game_screen.height(), self.size.1)),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
        self.root.flush();
    }
}

impl InputHandler for Window {
    fn wait_for_keypress(&mut self) -> Key {
        self.root.wait_for_keypress(true)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        
    }
}