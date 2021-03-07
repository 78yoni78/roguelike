use rand::prelude::*;

use crate::map::*;
use crate::pos::*;

/// A rectangle on the map, used to characterize a room.
#[derive(Clone, Copy, Debug)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: u16, h: u16) -> Self {
        Rect {
            x1: x,
            y1: y,
            x2: x + w as i32,
            y2: y + h as i32,
        }
    }

    pub fn center(&self) -> Pos {
        Pos::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }
    
    pub fn intersects_with(&self, other: &Rect) -> bool {
        // returns true if this rectangle intersects with another one
        (self.x1 <= other.x2)
            && (self.x2 >= other.x1)
            && (self.y1 <= other.y2)
            && (self.y2 >= other.y1)
    }
}

fn carve_room(room: Rect, map: &mut Map) {
    // go through the tiles in the rectangle and make them passable
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            let pos = Pos::new(x, y);
            map[pos] = Tile::Empty;
        }
    }
}

fn carve_horizontal_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    use std::cmp::{min, max};
    // horizontal tunnel. `min()` and `max()` are used in case `x1 > x2`
    for x in min(x1, x2)..(max(x1, x2) + 1) {
        let pos = Pos::new(x, y);
        map[pos] = Tile::Empty;
    }
}

fn carve_vertical_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    use std::cmp::{min, max};
    // vertical tunnel
    for y in min(y1, y2)..(max(y1, y2) + 1) {
        let pos = Pos::new(x, y);
        map[pos] = Tile::Empty;
    }
}

pub struct DungeonConfig {
    room_size: (u16, u16),
    max_rooms: u16,
    rng: rand::rngs::ThreadRng,
}

impl Default for DungeonConfig {
    fn default() -> Self {
        DungeonConfig {
            room_size: (6, 10),
            max_rooms: 30,
            rng: rand::thread_rng(),
        }
    }
}

fn generate_room(map_width: u16, map_height: u16, DungeonConfig { room_size: (room_size_min, room_size_max), rng, .. }: &mut DungeonConfig) -> Rect {
    // random width and height
    let w = rng.gen_range(*room_size_min..=*room_size_max);
    let h = rng.gen_range(*room_size_min..=*room_size_max);
    // random position without going out of the boundaries of the map
    let x = rng.gen_range(0..map_width - w);
    let y = rng.gen_range(0..map_height - h);

    Rect::new(x as i32, y as i32, w, h)
}

pub fn generate(width: u16, height: u16, config: &mut DungeonConfig) -> (Map, Pos) {
    let mut map = Map::new(width, height);
    for x in 0..width {
        for y in 0..height {
            map[Pos {x: x as i32, y: y as i32}] = Tile::Wall;
        }
    }

    let mut rooms = vec![];

    //  generate first room, place player
    let first_room = generate_room(width, height, config); 
    carve_room(first_room, &mut map);
    rooms.push(first_room);
    let player_pos = first_room.center();

    for _ in 1..config.max_rooms {
        let new_room = generate_room(width, height, config); 
        let intersection = rooms.iter().any(|other_room| new_room.intersects_with(other_room));

        if !intersection {
            carve_room(new_room, &mut map);
            // connect it to the previous room with a tunnel
            
            // center coordinates of the previous room
            let prev_room = &rooms[rooms.len() - 1];
            
            let (Pos {x: prev_x, y: prev_y}, Pos {x: new_x, y: new_y}) = (prev_room.center(), new_room.center());
            // toss a coin (random bool value -- either true or false)
            if rand::random() {                
                // first move horizontally, then vertically
                carve_horizontal_tunnel(prev_x, new_x, prev_y, &mut map);
                carve_vertical_tunnel(prev_y, new_y, new_x, &mut map);
            } else {
                // first move vertically, then horizontally
                carve_vertical_tunnel(prev_y, new_y, prev_x, &mut map);
                carve_horizontal_tunnel(prev_x, new_x, new_y, &mut map);
            }
        
            // finally, append the new room to the list
            rooms.push(new_room);
        }
    } 

    (map, player_pos)
}
