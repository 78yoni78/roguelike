use tcod::console::{Root, Offscreen};

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
}

impl Drop for Window {
    fn drop(&mut self) {
        
    }
}