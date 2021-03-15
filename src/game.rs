use std::{collections::HashMap, ops::Mul};

use crate::{dungeon_gen::{self, RectRoom}, pos::*};
use crate::input::{InputHandler, Key};
use crate::object::*;
use crate::object::{player::Player, enemy, enemy::Enemy};
use crate::map::{Map, Tile};
use crate::dungeon_gen::{Dungeon, DungeonConfig};
use tcod::map::Map as FovMap;
use tcod::Color;

fn tile_color(tile: Tile, darkened: bool) -> Option<Color> {
    use tcod::colors::*;

    let mut ret = match tile {
        Tile::Empty => None,
        Tile::Ground => Some(DARK_GREY),
        Tile::Wall => Some(LIGHTER_GREY),
    };
    if darkened {
        if let Some(mut color) = ret {
            color = color.mul(0.5);
            color.b += 100; //  must be less than 255 / 0.5 =~ 127
            ret = Some(color);
        }
    }
    ret
}


fn fov_map_from_map(map: &Map) -> FovMap {
    let mut fov_map = FovMap::new(map.width as i32, map.height as i32);
    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            match map[Pos{x, y}] {
                Tile::Empty => (),
                Tile::Wall => fov_map.set(x, y, false, false),
                Tile::Ground => fov_map.set(x, y, true, false),
            }
        }
    }
    fov_map
}

pub struct Game {
    pub player: Player,
    pub npcs: HashMap<u32, Enemy>,
    pub next_npc_id: u32,
    pub dungeon: Dungeon,
    pub map: Map,
    pub fov_map: FovMap,
}

impl Game {
    pub fn add_enemy(&mut self, enemy: Enemy) {
        self.npcs.insert(self.next_npc_id, enemy);
        self.next_npc_id += 1;
    }

    pub fn new() -> Self {
        let dungeon = DungeonConfig::default().generate();
        let map = dungeon.as_map();
        let player = Player::new(dungeon.rect_rooms[0].center(), 20);
        let fov_map = fov_map_from_map(&map);        

        let mut game = Game {
            player,
            dungeon,
            map,
            npcs: HashMap::new(),
            next_npc_id: 0,
            fov_map,
        };

        for rect_room in game.dungeon.rect_rooms.iter().skip(1).cloned().collect::<Vec<_>>() {
            game.add_enemy(enemy::basic_enemy(rect_room.center(), 5));
        }

        game
    }

    pub fn player_turn(&mut self, input: &mut dyn InputHandler) -> bool {
        use tcod::input::KeyCode::*;
        
        let key = input.wait_for_keypress();
        if key.code == Escape {
            return false;
        }

        if let Key { code: Char, printable: 'a', alt: false, ctrl: false, shift: false, pressed: true, .. } = key {
            self.player.attack(self.npcs.values_mut());
        }

        let mut target_pos = self.player.pos;
        match key {
            Key { code: Up, .. } => target_pos.move_by_inplace(0, -1),
            Key { code: Down, .. } => target_pos.move_by_inplace(0, 1),
            Key { code: Left, .. } => target_pos.move_by_inplace(-1, 0),
            Key { code: Right, .. } => target_pos.move_by_inplace(1, 0),
            _ => (),
        };

        if 0 <= target_pos.x
            && target_pos.y < self.map.width as i32
            && 0 <= target_pos.y
            && target_pos.y < self.map.height as i32
        {
            if self.map[target_pos] != Tile::Wall {
                self.player.pos = target_pos;
            }
        }

        return true;
    }

    pub fn npc_turn(&mut self) {
        for id in 0..self.next_npc_id {
            let mut npc = self.npcs.remove(&id).unwrap();
            npc.turn(self);
            self.npcs.insert(id, npc);
        }
    }

    fn draw_map(&mut self, con: &mut dyn tcod::Console) {
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let wall = self.map[Pos { x, y }];
                if let Some(color) = tile_color(wall, self.fov_map.is_in_fov(x, y)) {
                    con.set_char_background(x, y, color, tcod::BackgroundFlag::Set);
                }
            }
        }
    }

    pub fn draw_characters(&self, con: &mut dyn tcod::Console) {
        //  Draw state onto offscreen
        for (_, npc) in self.npcs.iter() {
            if self.fov_map.is_in_fov(npc.pos.x, npc.pos.y) {
                npc.draw(con);
            }
        }
        self.player.draw(con);
    }

    pub fn draw(&mut self, con: &mut dyn tcod::Console) {
        self.fov_map
        .compute_fov(self.player.pos.x, self.player.pos.y, 15, true, tcod::map::FovAlgorithm::Diamond);

        self.draw_map(con);
        self.draw_characters(con);
    }
}
