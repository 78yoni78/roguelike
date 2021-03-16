

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Pos {
    pub x: i32, 
    pub y: i32,
}

impl Pos {
    pub const fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub const fn move_by(self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    #[allow(dead_code)]
    pub const fn move_by_pos(self, other: Self) -> Self {
        Self::move_by(self, other.x, other.y)
    }

    pub fn move_by_inplace(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    #[allow(dead_code)]
    pub fn move_by_pos_inplace(&mut self, other: Self) {
        Self::move_by_inplace(self, other.x, other.y)
    }
}

