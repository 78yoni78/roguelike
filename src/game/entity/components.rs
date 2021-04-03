use tcod::Color;
use super::Entity;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UIPanel {
    Left, Bottom, Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UITransform {
    pub pos: (u16, u16),
    pub size: (u16, u16),
    pub panel: UIPanel,
}

#[derive(Debug)]
pub struct UIBar {
    pub label: &'static str,
    pub fill: f32,
    pub empty_color: Color,
    pub fill_color: Color,
}

#[derive(Debug, Clone)]
pub struct UIRect;

#[derive(Debug, Clone)]
pub struct UILabel(String);


pub type Components<T> = std::collections::HashMap<Entity, T>;

#[derive(Debug, Default)]
pub struct AllComponents {
    pub player: Option<(Entity, Player)>,
    pub enemies: Components<Enemy>,
    pub health: Components<Health>,
    pub draws: Components<Draw>,
    pub positions: Components<Position>,
    pub stuns: Components<Stun>,
    pub ui_transfroms: Components<UITransform>,
    pub ui_bars: Components<UIBar>,
    pub ui_rects: Components<UIRect>,
    pub ui_labels: Components<UILabel>,
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
        self.stuns.remove(&entity);
        self.ui_transfroms.remove(&entity);
        self.ui_bars.remove(&entity);
        self.ui_rects.remove(&entity);
        self.ui_labels.remove(&entity);
    }
}

#[allow(unused_macros)]
macro_rules! get {
    ($components:expr, $entity:expr, mut $field:ident) => (
        $components.$field.get_mut(&($entity))
    );
    ($components:expr, $entity:expr, $field:ident) => (
        $components.$field.get(&($entity))
    );
    ($components:expr, $entity:expr, $head:ident, $($rest: tt)+) => {
        if let Some(head) = get!($components, $entity, $head) {
            if let Some(tail) = get!($components, $entity, $($rest)+) {
                Some((head, tail))
            } else {
                None
            }
        } else {
            None
        }
    };
    ($components:expr, $entity:expr, mut $head:ident, $($rest: tt)+) => {
        if let Some(head) = get!($components, $entity, mut $head) {
            if let Some(tail) = get!($components, $entity, $($rest)+) {
                Some((head, tail))
            } else {
                None
            }
        } else {
            None
        }
    };
}

#[allow(unused_macros)]
macro_rules! get_mut {
    ($components:expr, $entity:expr, $field:ident) => (
        $components.$field.get_mut(&($entity))
    );
    ($components:expr, $entity:expr, $head:ident, $($tail:ident),+) => {
        if let Some(head) = get_mut!($components, $entity, $head) {
            if let Some(tail) = get_mut!($components, $entity, $($tail),+) {
                Some((head, tail))
            } else {
                None
            }
        } else {
            None
        }
    };
}

impl Draw {
    pub fn draw(&mut self, Position(x, y): &Position, tint: Color, con: &mut dyn tcod::Console) {
        use Draw::*;
        match self {
            Char(ch, color) => {
                con.set_default_foreground(*color * tint);
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

