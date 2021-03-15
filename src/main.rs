pub(self) mod input;
mod window;
mod game;

use window::Window;
use game::Game;

fn main() {
    const fps_limit: i32 = 20;
    const size: (u16, u16) = (80, 50);
    const title: &str = "My Game";

    //  Create a 
    let mut window = Window::new(window::Config { title: title.to_string(), size, fps_limit });
    let mut game = Game::new();

    while !window.closed() {
        window.redraw(|con| game.draw(con));

        game.player_turn(&mut window);

        game.npc_turn();
    }
}