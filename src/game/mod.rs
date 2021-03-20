pub(self) mod map;
mod dungeon_gen;
mod entity;

use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Mul;

use tcod::{Color, colors};

use crate::input::{InputHandler, Key, KeyCode};

use map::{Map, Tile, Pos};
use dungeon_gen::{Dungeon, DungeonConfig};
use entity::{
    Entity,
    entity_generator::EntityGenerator,
    // components::{self, AllComponents, Components},
    components::*,
};

type FovMap = tcod::map::Map;

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
    pub components: AllComponents,
    pub entity_gen: EntityGenerator,
    pub dungeon: Dungeon,
    pub map: Map,
    pub fov_map: FovMap,
    pub tiles_seen: HashSet<Pos>,
}

impl Game {
    fn remove(&mut self, entity: Entity) {
        self.entity_gen.remove(&entity);
        self.components.remove_all(entity);
    }

    fn remove_dead(&mut self) {
        let mut dead = vec![];
        for (&e, health) in &self.components.health {
            if health.hp == 0 {
                dead.push(e);
            }
        }
        for e in dead {
            self.remove(e);
        }
    }

    fn is_nearby(p1: &Position, p2: &Position) -> bool {
        (p1.0 as i32 - p2.0 as i32).abs() <= 1 &&
        (p1.1 as i32 - p2.1 as i32).abs() <= 1
    }

    fn nearby(position: &Position, positions: &Components<Position>) -> Vec<Entity> {
        let mut ret = vec![];
        for (&e, p) in positions {
            if Game::is_nearby(p, position) {
                   ret.push(e);
            }
        }
        ret
    }

    fn ai_turn(&mut self, e: Entity) {
        use EnemyMovement::*;
        let enemy = self.components.enemies[&e];
        if let &Some((player_entity, ref player)) = &self.components.player {
            if let Some(player_position) = &self.components.positions.get(&player_entity) {
            match enemy.movement {
                Simple => {
                    if Game::is_nearby(, p2) {

                    }
                },
            }
        }
    }
}

impl Game {
    pub fn generate_player(&mut self, pos: (u16, u16)) -> Entity {
        let e = self.entity_gen.generate();
        self.components.player = Some((e, Player{}));
        self.components.positions.insert(e, Position(pos.0, pos.1));
        self.components.health.insert(e, Health { hp: 10, ac: 0 });
        self.components.draws.insert(e, Draw::Char('@', colors::WHITE));
        e
    }

    pub fn generate_basic_enemy(&mut self, pos: (u16, u16)) -> Entity {
        let e = self.entity_gen.generate();
        self.components.positions.insert(e, Position(pos.0, pos.1));
        self.components.health.insert(e, Health { hp: 5, ac: 0 });
        self.components.draws.insert(e, Draw::Char('#', colors::YELLOW));
        self.components.enemies.insert(e, Enemy { movement: EnemyMovement::Simple });
        e
    }

    pub fn new() -> Self {
        let dungeon = DungeonConfig::default().generate();
        let map = dungeon.as_map();
        let fov_map = fov_map_from_map(&map);        
        
        let mut game = Game {
            entity_gen: EntityGenerator::new(),
            components: AllComponents::new(),
            dungeon,
            map,
            fov_map,
            tiles_seen: HashSet::new()
        };
        game.generate_player({ let p = game.dungeon.rect_rooms[0].center(); (p.x as u16, p.y as u16) });

        for rect_room in game.dungeon.rect_rooms.iter().skip(1).cloned().collect::<Vec<_>>() {
            game.generate_basic_enemy({let p = rect_room.center(); (p.x as u16, p.y as u16) });
        }

        game
    }

    pub fn player_attack(&mut self) {
        if let &Some((player_entity, ref player)) = &self.components.player {
            if let Some(position) = self.components.positions.get(&player_entity) {
                for nearby_entity in Self::nearby(position, &self.components.positions) {
                    if self.components.enemies.contains_key(&nearby_entity) {
                        if let Some(health) = self.components.health.get_mut(&nearby_entity) {
                            health.take_damage(2);
                        }
                    }
                }
            }
        }
    }
}

impl Game {
    pub fn player_turn(&mut self, input: &mut dyn InputHandler) -> bool {
        if let &Some((e, _)) = &self.components.player {
            use KeyCode::*;
            
            let key = input.wait_for_keypress();
            if key.code == Escape {
                return false;
            }

            if let Key { code: Char, printable: 'a', alt: false, ctrl: false, shift: false, pressed: true, .. } = key {
                self.player_attack();
                return true;
            }

            let mut target_pos = { let x = &self.components.positions[&e]; Pos::new(x.0 as i32, x.1 as i32) };
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
                    self.components.positions.get_mut(&e).unwrap().0 = target_pos.x as u16;
                    self.components.positions.get_mut(&e).unwrap().1 = target_pos.y as u16;
                }
            }

            return true;
        }
        return false;
    }

    pub fn npc_turn(&mut self) {
        self.remove_dead();

        for (&e, enemy) in &self.components.enemies {
            // enemy.turn();
            self.ai_turn();
        }
    }

    fn draw_map(&mut self, con: &mut dyn tcod::Console) {
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let wall = self.map[Pos { x, y }];
                
                let mut draw_darkened = None; 
                if self.fov_map.is_in_fov(x, y) {
                    self.tiles_seen.insert(Pos { x, y });
                    draw_darkened = Some(false);
                } else if self.tiles_seen.contains(&Pos { x, y }) {
                    draw_darkened = Some(true);
                }

                if let Some(darkened) = draw_darkened {
                    if let Some(color) = tile_color(wall, darkened) {
                        con.set_char_background(x, y, color, tcod::BackgroundFlag::Set);
                    }
                }
            }
        }
    }

    pub fn draw_characters(&mut self, con: &mut dyn tcod::Console) {
        //  Draw state onto offscreen
        for (&e, draw) in &mut self.components.draws {
            if let Some(position) = &self.components.positions.get(&e) {
                if self.fov_map.is_in_fov(position.0 as i32, position.1 as i32) {
                    draw.draw(position, con);
                }
            }
        }
    }

    pub fn draw(&mut self, con: &mut dyn tcod::Console) {
        if let &Some((e, _)) = &self.components.player {
            let player_pos = &self.components.positions[&e];
            
            self.fov_map
            .compute_fov(player_pos.0 as i32, player_pos.1 as i32, 15, true, tcod::map::FovAlgorithm::Diamond);
    
            self.draw_map(con);
            self.draw_characters(con);
        }
    }
}
