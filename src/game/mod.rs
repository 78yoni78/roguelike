pub(self) mod map;
mod dungeon_gen;
#[macro_use]
mod entity;

use std::collections::{HashSet, VecDeque};
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
    pub turn_order: VecDeque<Entity>,
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

    fn move_towards(&mut self, entity: Entity, target: (u16, u16)) -> Option<()> {
        use EnemyMovement::*;
        let (enemy, enemy_position) = get!(self.components, entity, enemies, mut positions)?;
        match enemy.movement {
            Simple => {
                let x_diff = target.0 as i32 - enemy_position.0 as i32;
                let y_diff = target.1 as i32 - enemy_position.1 as i32;
                let (dx, dy) = (x_diff.signum(), y_diff.signum());

                let mut new_position = enemy_position.clone();
                new_position.0 = (enemy_position.0 as i32 + dx) as u16;
                new_position.1 = (enemy_position.1 as i32 + dy) as u16;
                if self.map[Pos::new(new_position.0 as i32, new_position.1 as i32)] == Tile::Wall {
                    if self.map[Pos::new(new_position.0 as i32, enemy_position.1 as i32)] != Tile::Wall {
                        new_position.1 = enemy_position.1;
                    } else if self.map[Pos::new(enemy_position.0 as i32, new_position.1 as i32)] != Tile::Wall {
                        new_position.0 = enemy_position.0;
                    } else {
                        new_position = enemy_position.clone(); 
                    }
                }
                *enemy_position = new_position;
            },
        };
        Some(())
    }
}

impl Game {
    pub fn generate_player(&mut self, pos: (u16, u16)) -> Entity {
        let e = self.entity_gen.generate();
        self.components.player = Some((e, Player{}));
        self.components.positions.insert(e, Position(pos.0, pos.1));
        self.components.health.insert(e, Health { hp: 10, ac: 0 });
        self.components.draws.insert(e, Draw::Char('@', colors::WHITE));
        self.turn_order.push_back(e);
        e
    }

    pub fn generate_basic_enemy(&mut self, pos: (u16, u16)) -> Entity {
        let e = self.entity_gen.generate();
        self.components.positions.insert(e, Position(pos.0, pos.1));
        self.components.health.insert(e, Health { hp: 5, ac: 0 });
        self.components.draws.insert(e, Draw::Char('#', colors::YELLOW));
        self.components.enemies.insert(e, Enemy { movement: EnemyMovement::Simple });
        self.turn_order.push_back(e);
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
            tiles_seen: HashSet::new(),
            turn_order: VecDeque::new(),
        };
        game.generate_player({ let p = game.dungeon.rect_rooms[0].center(); (p.x as u16, p.y as u16) });

        for rect_room in game.dungeon.rect_rooms.iter().skip(1).cloned().collect::<Vec<_>>() {
            game.generate_basic_enemy({let p = rect_room.center(); (p.x as u16, p.y as u16) });
        }

        game
    }

    pub fn player_attack(&mut self) {
        if let &Some((player_entity, _)) = &self.components.player {
            if let Some(position) = self.components.positions.get(&player_entity) {
                for nearby_entity in Self::nearby(position, &self.components.positions) {
                    if self.components.enemies.contains_key(&nearby_entity) {
                        if let Some(health) = self.components.health.get_mut(&nearby_entity) {
                            health.take_damage(2);
                            if rand::random() && !self.components.stuns.contains_key(&nearby_entity) { self.components.stuns.insert(nearby_entity, Stun { duration: 5 }); }
                        }
                    }
                }
            }
        }
    }
}

impl Game {
    fn player_turn(&mut self, input: &mut dyn InputHandler) -> bool {
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

    fn npc_turn(&mut self, npc_entity: Entity) {
        let player_entity = self.components.player.as_ref().unwrap().0;
        let enemy_position = get!(self.components, npc_entity, positions).unwrap();
        let (player_position, player_health) = get!(self.components, player_entity, positions, mut health).unwrap();

        if let Some(stun) = get_mut!(self.components, npc_entity, stuns) {
            stun.duration -= 1;
            if stun.duration == 0 { self.components.stuns.remove(&npc_entity); }
        } else if Game::is_nearby(player_position, enemy_position) {
            player_health.take_damage(1);
        } else {
            let target = (player_position.0, player_position.1);
            self.move_towards(npc_entity, target);
        }
    }

    pub fn turn(&mut self, input: &mut dyn InputHandler) -> bool {
        let mut turn_entity = self.turn_order.pop_front().unwrap();
        while !self.entity_gen.contains(&turn_entity) {
            turn_entity = self.turn_order.pop_front().unwrap();
        }
        self.turn_order.push_back(turn_entity);
        let is_player_entity = self.components.player.iter().all(|x| x.0 == turn_entity);

        if is_player_entity {
            self.player_turn(input);
        } else {
            self.npc_turn(turn_entity);
        }
        self.remove_dead();
        match get!(self.components, turn_entity, positions) {
            None => true,
            Some(&Position(x,y)) => self.fov_map.is_in_fov(x as i32, y as i32),
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
            if let Some(position) = self.components.positions.get(&e) {
                if self.fov_map.is_in_fov(position.0 as i32, position.1 as i32) {
                    let tint = if self.components.stuns.contains_key(&e) { colors::DARK_RED } else { colors::WHITE };
                    draw.draw(position, tint, con);
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
