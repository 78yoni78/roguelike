use std::collections::HashMap;

use crate::input::{InputHandler, Key};
use crate::object::{player::Player, enemy, enemy::Enemy};
use crate::map::{Map, Tile};
use crate::dungeon_gen::{Dungeon, DungeonConfig};

pub struct Game {
    player: Player,
    npcs: HashMap<u32, Enemy>,
    next_npc_id: u32,
    dungeon: Dungeon,
    map: Map,
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
        
        let mut game = Game {
            player,
            dungeon,
            map,
            npcs: HashMap::new(),
            next_npc_id: 0,
        };

        for rect_room in &dungeon.rect_rooms.iter().skip(1) {
            game.add_enemy(enemy::basic_enemy(rect_room.center(), 5));
        }

        game
    }

    pub fn player_turn(&mut self, input: &mut dyn InputHandler) {
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
            npc.turn(&mut self);
            self.npcs.insert(id, npc);
        }
    }
}
