use super::*;

pub struct Player {
    pub pos: Pos,
    pub health: Health,
}

impl Player {
    const COLOR: Color = colors::WHITE;
    const CH: char = '@';

    pub fn new(pos: Pos, max_hp: u32) -> Self {
        Player { pos, health: Health { hp: max_hp, ac: 0 } }
    }
}

impl Draw for Player {
    fn draw(&self, con: &mut dyn tcod::Console) {
        con.set_default_foreground(Self::COLOR);
        con.put_char(self.pos.x, self.pos.y, Self::CH, tcod::BackgroundFlag::None);
    }
}