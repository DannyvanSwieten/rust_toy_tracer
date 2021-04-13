use super::types::*;

pub struct Intersection {
    pub position: Position,
    pub in_direction: Direction,
    pub t: f32,
    pub primitive_id: u32,
    pub barycentrics: Barycentrics,
}

impl Intersection {
    pub fn new(
        position: &Position,
        in_direction: &Direction,
        t: f32,
        primitive_id: u32,
        barycentrics: &Barycentrics,
    ) -> Self {
        Self {
            position: *position,
            in_direction: *in_direction,
            t,
            primitive_id,
            barycentrics: *barycentrics,
        }
    }
}
