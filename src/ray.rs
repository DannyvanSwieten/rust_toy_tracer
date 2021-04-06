use super::types::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Position,
    pub dir: Direction,
}

impl Ray {
    pub fn new(origin: &Position, direction: &Direction) -> Self {
        return Self {
            origin: *origin,
            dir: *direction,
        };
    }

    pub fn origin(&self) -> &Position {
        &self.origin
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }

    pub fn at(&self, t: f32) -> Position {
        self.origin + self.dir * t
    }
}
