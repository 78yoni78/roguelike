mod pos;

use std::ops::{Index, IndexMut};

pub use pos::Pos;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Ground,
}

pub struct Map {
    pub width: u16,
    pub height: u16,
    tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: u16, height: u16) -> Self {
        Map { width, height, tiles: vec![Tile::Empty; width as usize * height as usize] }
    }

    pub fn tiles(&self) -> impl Iterator<Item=&Tile> {
        self.tiles.iter()
    }

    pub fn tiles_mut(&mut self) -> impl Iterator<Item=&mut Tile> {
        self.tiles.iter_mut()
    }
}

impl Index<Pos> for Map {
    type Output = Tile;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.tiles[index.y as usize * self.width as usize + index.x as usize] 
    }
}

impl IndexMut<Pos> for Map {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.tiles[index.y as usize * self.width as usize + index.x as usize] 
    }
}

