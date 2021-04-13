use super::types::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Position,
    pub dir: Direction,
    pub inv_dir: Direction,
}

impl Ray {
    pub fn new(origin: &Position, direction: &Direction) -> Self {
        return Self {
            origin: *origin,
            dir: *direction,
            inv_dir: Direction::from_values(&[1., 1., 1.]) / direction,
        };
    }

    pub fn origin(&self) -> &Position {
        &self.origin
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }

    pub fn inv_direction(&self) -> &Direction {
        &self.inv_dir
    }

    pub fn at(&self, t: f32) -> Position {
        self.origin + self.dir * t
    }
}
