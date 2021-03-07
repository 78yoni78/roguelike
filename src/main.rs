pub mod map;
pub mod dungeon_gen;
pub mod object;
pub mod pos;

use std::collections::hash_map::HashMap;

use dungeon_gen::DungeonConfig;
use tcod::{colors, colors::Color, console::*, input::Key};

use map::*;
use object::*;
use object::{enemy::Enemy, player::Player};
use pos::*;

type Map = map::Map;

pub struct Tcod {
    screen_size: Pos,
    root: Root,
    con: Offscreen,
}

impl Tcod {
    pub fn new(state: &State, screen_size: Pos) -> Self {
        let root = Root::initializer()
            //  .font("consolas_unicode_16x16.png", FontLayout::Tcod)
            .font("arial10x10.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(screen_size.x, screen_size.y)
            .title("A Rogue-like!")
            .init();

        let con = Offscreen::new(state.map.width as i32, state.map.height as i32);

        Tcod {
            screen_size,
            root,
            con,
        }
    }
}

pub struct State {
    player: Player,
    npcs: HashMap<u32, Enemy>,
    next_npc_id: u32,
    map: Map,
}

impl State {
    pub fn new(map_width: u16, map_height: u16) -> Self {
        let player = Player::new(Pos::new(map_width as i32 / 2, map_height as i32 / 2), 10);
        let npcs = [
            (0, enemy::basic_enemy(player.pos.move_by(5, 1), 10))
        ].iter().cloned().collect(); 
        let map = Map::new(map_width, map_height);
        State { player, npcs, next_npc_id: 1, map }
    }

    pub fn draw_characters(&self, con: &mut dyn Console) {
        //  Draw state onto offscreen
        for (_, npc) in self.npcs.iter() {
            npc.draw(con);
        }
        self.player.draw(con);
    }

    pub fn draw_map(&self, tile_color: fn(Tile) -> Option<Color>, con: &mut dyn Console) {
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let wall = self.map[Pos { x, y }];
                if let Some(color) = tile_color(wall) {
                    con.set_char_background(x, y, color, BackgroundFlag::Set);
                }
            }
        }
    }
}

fn input_dispatch(state: &mut State, key: Key) -> bool {
    use tcod::input::KeyCode::*;

    if key.code == Escape {
        return false;
    }

    let mut target_pos = state.player.pos;
    match key {
        Key { code: Up, .. } => target_pos.move_by_inplace(0, -1),
        Key { code: Down, .. } => target_pos.move_by_inplace(0, 1),
        Key { code: Left, .. } => target_pos.move_by_inplace(-1, 0),
        Key { code: Right, .. } => target_pos.move_by_inplace(1, 0),
        _ => (),
    };

    if 0 <= target_pos.x
        && target_pos.y < state.map.width as i32
        && 0 <= target_pos.y
        && target_pos.y < state.map.height as i32
    {
        if state.map[target_pos] != Tile::Wall {
            state.player.pos = target_pos;
        }
    }

    return true;
}

fn tile_color(tile: Tile) -> Option<Color> {
    match tile {
        Tile::Empty => None,
        Tile::Ground => Some(colors::DARK_GREY),
        Tile::Wall => Some(colors::GREY),
    }
}

const LIMIT_FPS: i32 = 20;

fn main() {
    let mut state = State::new(80, 45);
    let mut tcod = Tcod::new(&state, Pos::new(80, 50));

    let dungeon = {
        let mut c = DungeonConfig::default();
        c.size = (80, 45);
        c.generate()
    };
    let map = dungeon.as_map(); 
    state.map = map;
    state.player.pos = dungeon.rect_rooms[0].center();
    //for x in 0..std::cmp::min(state.map.width, state.map.height) as i32 {
    //    state.map[Pos::new(x, x)] = Tile::Wall;
    //}

    tcod::system::set_fps(LIMIT_FPS);

    while !tcod.root.window_closed() {
        //  Clear the offscreen
        tcod.con.clear();

        state.draw_map(tile_color, &mut tcod.con);
        state.draw_characters(&mut tcod.con);

        //  Draw the offscreen onto the root screen and flush
        blit(
            &tcod.con,
            (0, 0),
            (tcod.screen_size.x, tcod.screen_size.y),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
        tcod.root.flush();

        //  Input handling
        let key = tcod.root.wait_for_keypress(true);
        let success = input_dispatch(&mut state, key);
        if !success {
            break;
        }

        for id in 0..state.next_npc_id {
            let mut npc = state.npcs.remove(&id).unwrap();
            npc.turn(&mut state);
            state.npcs.insert(id, npc);
        }
    }
}
