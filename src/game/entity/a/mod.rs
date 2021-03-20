mod player;
mod enemy;

use tcod::Color;

pub use enemy::*;
pub use player::*;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub struct Entity(u32);

#[derive(Debug, Clone)]
pub enum Draw {
    Char(char, Color),
}

#[derive(Debug, Clone)]
pub struct Health {
    pub hp: u32,
    pub ac: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position(pub u16, pub u16);

pub type Components<T> = std::collections::HashMap<Entity, T>;

pub struct AllComponents {
    pub player: Player,
    pub enemies: Components<Enemy>,
    pub health: Components<Health>,
    pub draws: Components<Draw>,
    pub positions: Components<Position>,
}


impl Draw {
    fn draw(&mut self, Position(x, y): Position, con: &mut dyn tcod::Console) {
        use Draw::*;
        match self {
            Char(ch, color) => {
                con.set_default_foreground(color);
                con.set_char(x, y, ch);
            },
        }
    }
}

impl Health {
    fn take_damage(&mut self, damage: u32) {
        if damage <= self.ac {
            return;
        }
        let hp_taken = std::cmp::min(damage - self.ac, self.hp);    //  because you cant damage an enemy more than their hp
        self.hp -= hp_taken;
    }
}