pub mod pos;
pub mod map;

use std::iter::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

use pos::*;
use map::*;

type Map = map::Map;

fn tile_color(tile: Tile) -> Option<Color> {
    match tile {
        Tile::Empty => None,
        Tile::Ground => Some(BLUE),
        Tile::Wall => Some(RED),
    }
}

struct Tcod {
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
        .title("A Rougelike!")
        .init();

        let con = Offscreen::new(state.map.width as i32, state.map.height as i32);

        Tcod { screen_size, root, con }
    }
}

struct Object {
    pos: Pos,
    ch: char,
    color: Color,
}

impl Object {
    pub const fn new(pos: Pos, ch: char, color: Color) -> Self {
        Object { pos, ch, color }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.pos.x, self.pos.y, self.ch, BackgroundFlag::None);
    }
}

struct State {
    player: Object,
    npcs: Vec<Object>,
    map: Map,
}

impl State {
    pub fn new (map_width: u16, map_height: u16) -> Self {
        let player = Object::new(Pos::new(map_width as i32 / 2, map_height as i32 / 2), '@', WHITE);
        let npcs = vec![ Object::new(player.pos.move_by(5, 1), '#', YELLOW) ];
        let map = Map::new(map_width, map_height);
        State { player, npcs, map }
    }

    pub fn objects(&self) -> impl Iterator<Item = &Object> {
        once(&self.player).chain(self.npcs.iter())
    }

    pub fn objects_mut(&mut self) -> impl Iterator<Item = &mut Object> {
        once(&mut self.player).chain(self.npcs.iter_mut())
    }

    pub fn draw_objects(&self, con: &mut dyn Console) {
        //  Draw state onto offscreen
        for object in self.objects() {
            object.draw(con);
        }
    }

    pub fn draw_map(&self, con: &mut dyn Console) {
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
    if key.code == Escape {
        return false;
    }

    let mut pos = state.player.pos;
    match key {
        Key { code: Up, .. } => pos.move_by_inplace(0, -1),
        Key { code: Down, .. } => pos.move_by_inplace(0, 1),
        Key { code: Left, .. } => pos.move_by_inplace(-1, 0),
        Key { code: Right, .. } => pos.move_by_inplace(1, 0),
        _ => (),
    };

    if 0 <= pos.x && pos.y < state.map.width as i32 && 0 <= pos.y && pos.y < state.map.height as i32 {
        if state.map[pos] == Tile::Empty {
            state.player.pos = pos;
        }
    }

    return true;
}


const LIMIT_FPS: i32 = 20;

fn main() {
    let mut state = State::new(80, 45);
    let mut tcod = Tcod::new(&state, Pos::new(80, 50));

    for x in 0..std::cmp::min(state.map.width, state.map.height) as i32 {
        state.map[Pos::new(x, x)] = Tile::Wall;
    }

    tcod::system::set_fps(LIMIT_FPS);
    
    while !tcod.root.window_closed() {
        //  Clear the offscreen
        tcod.con.clear();

        state.draw_map(&mut tcod.con);
        state.draw_objects(&mut tcod.con);

        //  Draw the offscreen onto the root screen and flush
        blit(&tcod.con, (0, 0), (tcod.screen_size.x, tcod.screen_size.y), &mut tcod.root, (0, 0), 1.0, 1.0);
        tcod.root.flush();

        //  Input handling
        let key = tcod.root.wait_for_keypress(true);
        let success = input_dispatch(&mut state, key); 
        if !success {
            break;
        }
    }
}

