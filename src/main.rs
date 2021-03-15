pub mod map;
pub mod dungeon_gen;
pub mod object;
pub mod pos;

use std::{collections::hash_map::HashMap, str::FromStr};

use dungeon_gen::DungeonConfig;
use tcod::{colors, colors::Color, console::*, input::Key};

use map::*;
use object::*;
use object::{enemy::Enemy, player::Player};
use pos::*;

type Map = map::Map;
type FovMap = tcod::map::Map;

// pub struct Tcod {
//     screen_size: Pos,
//     root: Root,
//     con: Offscreen,
//     fov_map: FovMap,
// }

// impl Tcod {
//     pub fn new(state: &State, screen_size: Pos, map: &Map) -> Self {
//         let root = Root::initializer()
//             //  .font("consolas_unicode_16x16.png", FontLayout::Tcod)
//             .font("arial10x10.png", FontLayout::Tcod)
//             .font_type(FontType::Greyscale)
//             .size(screen_size.x, screen_size.y)
//             .title("A Rogue-like!")
//             .init();

//         let con = Offscreen::new(state.map.width as i32, state.map.height as i32);

//         let mut fov_map = tcod::map::Map::new(map.width as i32, map.height as i32);
//         for y in 0..map.height as i32 {
//             for x in 0..map.width as i32 {
//                 match map[Pos{x, y}] {
//                     Tile::Empty => (),
//                     Tile::Wall => fov_map.set(x, y, false, false),
//                     Tile::Ground => fov_map.set(x, y, true, false),
//                 }
//             }
//         }

//         Tcod {
//             screen_size,
//             root,
//             con,
//             fov_map,
//         }
//     }
// }

// pub struct State {
//     player: Player,
//     npcs: HashMap<u32, Enemy>,
//     next_npc_id: u32,
//     map: Map,
// }

// impl State {
//     pub fn new(map_width: u16, map_height: u16) -> Self {
//         let player = Player::new(Pos::new(map_width as i32 / 2, map_height as i32 / 2), 10);
//         let npcs = [
//             (0, enemy::basic_enemy(player.pos.move_by(5, 1), 10))
//         ].iter().cloned().collect(); 
//         let map = Map::new(map_width, map_height);
//         State { player, npcs, next_npc_id: 1, map }
//     }

//     pub fn draw_characters(&self, con: &mut dyn Console, fov_map: &FovMap) {
//         //  Draw state onto offscreen
//         for (_, npc) in self.npcs.iter() {
//             if fov_map.is_in_fov(npc.pos.x, npc.pos.y) {
//                 npc.draw(con);
//             }
//         }
//         self.player.draw(con);
//     }

//     pub fn draw_map(&self, tile_color: fn(Tile, bool) -> Option<Color>, con: &mut dyn Console, fov_map: &FovMap) {
//         for y in 0..self.map.height as i32 {
//             for x in 0..self.map.width as i32 {
//                 let wall = self.map[Pos { x, y }];
//                 if let Some(color) = tile_color(wall, fov_map.is_in_fov(x, y)) {
//                     con.set_char_background(x, y, color, BackgroundFlag::Set);
//                 }
//             }
//         }
//     }
// }

// fn input_dispatch(state: &mut State, key: Key) -> bool {
//     use tcod::input::KeyCode::*;

//     if key.code == Escape {
//         return false;
//     }

//     if let Key { code: Char, printable: 'a', alt: false, ctrl: false, shift: false, pressed: true, .. } = key {
//         state.player.attack(state.npcs.values_mut());
//     }

//     let mut target_pos = state.player.pos;
//     match key {
//         Key { code: Up, .. } => target_pos.move_by_inplace(0, -1),
//         Key { code: Down, .. } => target_pos.move_by_inplace(0, 1),
//         Key { code: Left, .. } => target_pos.move_by_inplace(-1, 0),
//         Key { code: Right, .. } => target_pos.move_by_inplace(1, 0),
//         _ => (),
//     };

//     if 0 <= target_pos.x
//         && target_pos.y < state.map.width as i32
//         && 0 <= target_pos.y
//         && target_pos.y < state.map.height as i32
//     {
//         if state.map[target_pos] != Tile::Wall {
//             state.player.pos = target_pos;
//         }
//     }

//     return true;
// }

// fn tile_color(tile: Tile, visible: bool) -> Option<Color> {
//     match (tile, visible) {
//         (Tile::Empty, _) => None,
//         (Tile::Ground, true) => Some(colors::DARK_GREY),
//         (Tile::Ground, false) => None, //Some(colors::DARKER_GREY),
//         (Tile::Wall, true) => Some(colors::LIGHTER_GREY),
//         (Tile::Wall, false) => None, //Some(colors::GREY),
//     }
// }

// const LIMIT_FPS: i32 = 20;

// fn main() {
//     let mut state = State::new(80, 45);
//     let dungeon = {
//         let mut c = DungeonConfig::default();
//         c.size = (80, 45);
//         c.generate()
//     };
//     let map = dungeon.as_map(); 
//     state.map = map;
//     state.player.pos = dungeon.rect_rooms[0].center();
//     //for x in 0..std::cmp::min(state.map.width, state.map.height) as i32 {
//     //    state.map[Pos::new(x, x)] = Tile::Wall;
//     //}
//     let mut tcod = Tcod::new(&state, Pos::new(80, 50), &state.map);


//     tcod::system::set_fps(LIMIT_FPS);

//     while !tcod.root.window_closed() {
//         //  Clear the offscreen
//         tcod.con.clear();
        
//         tcod.fov_map
//         .compute_fov(state.player.pos.x, state.player.pos.y, 15, true, tcod::map::FovAlgorithm::Diamond);

//         state.draw_map(tile_color, &mut tcod.con, &tcod.fov_map);
//         state.draw_characters(&mut tcod.con, &tcod.fov_map);

//         //  Draw the offscreen onto the root screen and flush
//         blit(
//             &tcod.con,
//             (0, 0),
//             (tcod.screen_size.x, tcod.screen_size.y),
//             &mut tcod.root,
//             (0, 0),
//             1.0,
//             1.0,
//         );
//         tcod.root.flush();

//         //  Input handling
//         let key = tcod.root.wait_for_keypress(true);
//         let success = input_dispatch(&mut state, key);
//         if !success {
//             break;
//         }

//         for id in 0..state.next_npc_id {
//             let mut npc = state.npcs.remove(&id).unwrap();
//             npc.turn(&mut state);
//             state.npcs.insert(id, npc);
//         }
//     }
// }

mod window;
mod game;
pub mod input;

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