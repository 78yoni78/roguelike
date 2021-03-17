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
    ui_screen: Offscreen,
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
        let ui_screen = Offscreen::new(config.size.0 as i32, 5);

        tcod::system::set_fps(config.fps_limit);
    
        Window { config, root, game_screen, ui_screen }
    }

    pub fn closed(&self) -> bool {
        self.root.window_closed()
    }

    pub fn redraw<F: FnMut(&mut dyn Console)>(&mut self, mut func: F) {
        use std::cmp::min;

        //  Clear the offscreen
        self.game_screen.clear();
        func(&mut self.game_screen);
        //  Clear the offscreen
        self.ui_screen.set_default_background(tcod::colors::DARK_GREY);
        self.ui_screen.clear();
        {
            let value = 7;
            let maximum = 15;
            let total_width = 50;
            let back_color = tcod::colors::RED;
            let bar_color = tcod::colors::BLUE;
            let (x, y) = (5, 2);
            let name = "Mana";

            // render a bar (HP, experience, etc). First calculate the width of the bar
            let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

            // render the background first
            self.ui_screen.set_default_background(back_color);
            self.ui_screen.rect(x, y, total_width, 1, false, tcod::BackgroundFlag::Overlay);

            // now render the bar on top
            self.ui_screen.set_default_background(bar_color);
            if bar_width > 0 {
                self.ui_screen.rect(x, y, bar_width, 1, false, tcod::BackgroundFlag::Overlay);
            }
            // finally, some centered text with the values
            self.ui_screen.set_default_foreground(tcod::colors::WHITE);
            self.ui_screen.print_ex(
                x + total_width / 2,
                y,
                tcod::BackgroundFlag::None,
                tcod::TextAlignment::Center,
                &format!("{}: {}/{}", name, value, maximum),
            );
        }
        //  Draw the offscreen onto the root screen and flush
        tcod::console::blit(
            &self.game_screen,
            (0, 0),
            (min(self.game_screen.width(), self.config.size.0 as i32), min(self.game_screen.height(), self.config.size.1 as i32)),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
        let h = self.root.height();
        tcod::console::blit(
            &self.ui_screen,
            (0, 0),
            (min(self.ui_screen.width(), self.config.size.0 as i32), min(self.game_screen.height(), self.config.size.1 as i32)),
            &mut self.root,
            (0, h - 5),
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