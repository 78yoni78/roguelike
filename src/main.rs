pub(self) mod input;
mod window;
mod game;
mod ui;

use window::Window;
use game::Game;

fn main() {
    const FPS_LIMIT: i32 = 20;
    const SIZE: (u16, u16) = (80, 50);
    const TITLE: &str = "My Game";

    let mut window = Window::new(window::Config {
        title: TITLE.to_string(),
        size: SIZE,
        fps_limit: FPS_LIMIT
    });
    let mut game = Game::new();

    while !window.closed() {
        window.redraw(|con| game.draw(con));

        game.player_turn(&mut window);

        game.npc_turn();
    }
}
