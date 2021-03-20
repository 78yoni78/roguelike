use tcod::Color;
use super::Entity;

pub trait Turn<Input> {
    fn turn(&mut self, input: Input);
}

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

#[derive(Debug, Clone)]
pub enum EnemyMovement {
    Simple,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub movement: EnemyMovement,
}

#[derive(Debug, Clone)]
pub struct Stun {
    pub duration: u16,
}

#[derive(Debug)]
pub struct Player {}


pub type Components<T> = std::collections::HashMap<Entity, T>;

#[derive(Debug, Default)]
pub struct AllComponents {
    pub player: Option<(Entity, Player)>,
    pub enemies: Components<Enemy>,
    pub health: Components<Health>,
    pub draws: Components<Draw>,
    pub positions: Components<Position>,
    pub stuns: Components<Stun>,
}

impl AllComponents {
    pub fn new() -> Self { Self::default() }

    pub fn remove_all(&mut self, entity: Entity) {
        if let Some((e, _)) = self.player {
            if e == entity { self.player = None; }
        }
        self.enemies.remove(&entity);
        self.health.remove(&entity);
        self.draws.remove(&entity);
        self.positions.remove(&entity);
    }
}

#[allow(unused_macros)]
macro_rules! get {
    ($components:expr, $entity:expr, $field:ident) => (
        $components.$field.get(&($entity))
    );
    ($components:expr, $entity:expr, $field1:ident, $($fields: ident),+) => {
        (get!($components, $entity, $field1),
        get!($components, $entity, $($fields),+))
    };
}

fn f(components: &AllComponents, entity: Entity) {
    let x = get!(components, entity, positions, health);
}


impl Draw {
    pub fn draw(&mut self, Position(x, y): &Position, con: &mut dyn tcod::Console) {
        use Draw::*;
        match self {
            Char(ch, color) => {
                con.set_default_foreground(*color);
                con.put_char(*x as i32, *y as i32, *ch, tcod::BackgroundFlag::None);
            },
        }
    }
}

impl Health {
    pub fn take_damage(&mut self, damage: u32) {
        if damage <= self.ac {
            return;
        }
        let hp_taken = std::cmp::min(damage - self.ac, self.hp);    //  because you cant damage an enemy more than their hp
        self.hp -= hp_taken;
    }
}

impl Turn<()> for Stun {
    fn turn(&mut self, (): ()) {
        self.duration -= 1;
    }
}