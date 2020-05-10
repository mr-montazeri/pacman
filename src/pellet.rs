use crate::world::Position;

pub struct Pellet {
    pos: Position,
    value: i32,
}

impl Pellet {
    pub fn new(pos: Position, value: i32) -> Self {
        Pellet { pos, value }
    }

    pub fn pos(&self) -> Position {
        self.pos
    }
}