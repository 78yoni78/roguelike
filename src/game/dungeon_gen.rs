use rand::prelude::*;

use super::map::{Map, Tile, Pos};

#[derive(Debug, Clone, Copy)]
pub struct RectRoom {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct HCorridor {
    pub x1: i32,
    pub x2: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct VCorridor {
    pub y1: i32,
    pub y2: i32,
    pub x: i32,
}

pub struct DungeonConfig {
    pub size: (u16, u16),
    pub room_size: (u16, u16),
    pub max_rooms: u16,
    pub rng: rand::rngs::ThreadRng,
}

pub struct Dungeon {
    config: DungeonConfig,
    pub rect_rooms: Vec<RectRoom>,
    pub h_corridors: Vec<HCorridor>,
    pub v_corridors: Vec<VCorridor>,
}


impl RectRoom {
    pub fn new(x: i32, y: i32, w: u16, h: u16) -> Self {
        RectRoom {
            x1: x,
            y1: y,
            x2: x + w as i32,
            y2: y + h as i32,
        }
    }

    pub fn center(&self) -> Pos {
        Pos::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
    
    pub fn intersects_with(&self, other: &Self) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

impl DungeonConfig {
    fn random_rect_room(&mut self, map_width: u16, map_height: u16) -> RectRoom {
        let DungeonConfig { room_size: (room_size_min, room_size_max), rng, .. } = self;

        // random width and height
        let w = rng.gen_range(*room_size_min..=*room_size_max);
        let h = rng.gen_range(*room_size_min..=*room_size_max);
        // random position without going out of the boundaries of the map
        let x = rng.gen_range(0..map_width - w);
        let y = rng.gen_range(0..map_height - h);

        RectRoom::new(x as i32, y as i32, w, h)
    }
}

impl Default for DungeonConfig {
    fn default() -> Self {
        DungeonConfig {
            size: (80, 45),
            room_size: (6, 10),
            max_rooms: 30,
            rng: rand::thread_rng(),
        }
    }
}


trait Room {
    fn carve_walls(&self, map: &mut Map);
    fn carve_floors(&self, map: &mut Map);
}

impl Room for RectRoom {
    fn carve_walls(&self, map: &mut Map) {
        for x in self.x1..=self.x2 {
            map[Pos { x, y: self.y1 }] = Tile::Wall;    
            map[Pos { x, y: self.y2 }] = Tile::Wall;    
        }
        for y in self.y1..=self.y2 {
            map[Pos { y, x: self.x1 }] = Tile::Wall;    
            map[Pos { y, x: self.x2 }] = Tile::Wall;    
        }
    }
    fn carve_floors(&self, map: &mut Map) {
        for x in self.x1+1..self.x2 {
            for y in self.y1+1..self.y2 {
                let pos = Pos::new(x, y);
                map[pos] = Tile::Ground;
            }
        }
    }
}

impl Room for HCorridor {
    fn carve_walls(&self, map: &mut Map) {
        for y in self.y-1..=self.y+1 {
            if 0 <= y && y < map.height as i32 {
                for x in self.x1-1..=self.x2+1 {
                    map[Pos {x, y}] = Tile::Wall;
                }
            }
        }
    }

    fn carve_floors(&self, map: &mut Map) {
        for x in self.x1..=self.x2 {
            map[Pos {x, y: self.y}] = Tile::Ground;
        }
    }
}

impl Room for VCorridor {
    fn carve_walls(&self, map: &mut Map) {
        for x in self.x-1..=self.x+1 {
            if 0 <= x && x < map.width as i32 {
                for y in self.y1-1..=self.y2+1 {
                    map[Pos {x, y}] = Tile::Wall;
                }
            }
        }
    }

    fn carve_floors(&self, map: &mut Map) {
        for y in self.y1..=self.y2 {
            map[Pos {y, x: self.x}] = Tile::Ground;
        }
    }
}


impl DungeonConfig {
    pub fn generate(mut self) -> Dungeon {
        let mut rect_rooms = vec![];
        let mut h_corridors = vec![];
        let mut v_corridors = vec![];
    
        //  Add rect rooms
        for _ in 0..self.max_rooms {
            let new_rect = self.random_rect_room(self.size.0, self.size.1); 
            let intersection = rect_rooms.iter().any(|other_rect| new_rect.intersects_with(other_rect));
            if !intersection {
                rect_rooms.push(new_rect);
            }
        }
    
        for i in 1..rect_rooms.len() {
            use std::cmp::{min, max};
            let prev = rect_rooms[i - 1];
            let next = rect_rooms[i];
    
            let (Pos {x: prev_x, y: prev_y}, Pos {x: next_x, y: next_y}) = (prev.center(), next.center());
            if rand::random() {                
                // first move horizontally, then vertically
                h_corridors.push(HCorridor{
                    x1: min(prev_x, next_x), x2: max(prev_x, next_x), y: prev_y
                });
                v_corridors.push(VCorridor{
                    y1: min(prev_y, next_y), y2: max(prev_y, next_y), x: next_x
                });
            } else {
                // first move vertically, then horizontally
                v_corridors.push(VCorridor{
                    y1: min(prev_y, next_y), y2: max(prev_y, next_y), x: prev_x
                });
                h_corridors.push(HCorridor{
                    x1: min(prev_x, next_x), x2: max(prev_x, next_x), y: next_y
                });
            }
        }
    
        Dungeon { config: self, rect_rooms, h_corridors, v_corridors }
    }
}

impl Dungeon {
    #[allow(dead_code)]
    pub fn config(&self) -> &DungeonConfig { &self.config }
    
    #[allow(dead_code)]
    pub fn size(&self) -> (u16, u16) { self.config.size }
    
    pub fn width(&self) -> u16 { self.config.size.0 }
    pub fn height(&self) -> u16 { self.config.size.1 }

    fn rooms(&self) -> impl Iterator<Item=&dyn Room> {
        self.rect_rooms.iter().map(|x| x as &dyn Room)
        .chain(self.h_corridors.iter().map(|x| x as &dyn Room))
        .chain(self.v_corridors.iter().map(|x| x as &dyn Room))
    }

    pub fn as_map(&self) -> Map {
        let mut map = Map::new(self.width(), self.height());
    
        for room in self.rooms() {
            room.carve_walls(&mut map);
        }
        for room in self.rooms() {
            room.carve_floors(&mut map);
        }
    
        map
    }
}
