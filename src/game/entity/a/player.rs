use super::*;
use super::enemy::Enemy;

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

    fn can_reach(&mut self, target: Pos) -> bool {
        (self.pos.x - target.x).abs() <= 1 &&
        (self.pos.y - target.y).abs() <= 1
    }
    
    pub fn attack<'a, I: IntoIterator<Item=&'a mut Enemy>>(&mut self, npcs: I) {
        for npc in npcs {
            if self.can_reach(npc.pos) {
                npc.health.take_damage(1);
                if rand::random() { npc.stun(); }
            }
        }
    }

}

impl Draw for Player {
    fn draw(&self, con: &mut dyn tcod::Console) {
        con.set_default_foreground(Self::COLOR);
        con.put_char(self.pos.x, self.pos.y, Self::CH, tcod::BackgroundFlag::None);
    }
}