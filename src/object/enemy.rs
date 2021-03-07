use super::*;
use crate::map::Tile;

#[derive(Clone)]
enum Movement {
    Simple,
}

#[derive(Clone)]
pub struct Enemy { 
    pub pos: Pos,
    pub health: Health,
    color: Color,
    ch: char,
    movement: Movement,
}

impl Draw for Enemy {
    fn draw(&self, con: &mut dyn tcod::Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.pos.x, self.pos.y, self.ch, tcod::BackgroundFlag::None);
    }
}

impl Turn for Enemy {
    fn turn(&mut self, state: &mut State) {
        use Movement::*;
        match self.movement {
            Simple => {
                let x_diff = state.player.pos.x - self.pos.x;
                let y_diff = state.player.pos.y - self.pos.y;
                let (dx, dy) = (x_diff.signum(), y_diff.signum());

                let new_pos = self.pos.move_by(dx, dy);
                if state.map[new_pos] != Tile::Wall {
                    self.pos = new_pos;
                } else {
                    let new_pos = self.pos.move_by(dx, 0);
                    if state.map[new_pos] != Tile::Wall {
                        self.pos = new_pos;
                    } else {
                        let new_pos = self.pos.move_by(0, dy);
                        if state.map[new_pos] != Tile::Wall {
                            self.pos = new_pos;
                        }
                    }
                }
            }
        }
    }
}

pub fn basic_enemy(pos: Pos, hp: u32) -> Enemy {
    Enemy {
        pos,
        health: Health { hp, ac: 0 },
        color: colors::YELLOW,
        ch: '#',
        movement: Movement::Simple,
    }
}