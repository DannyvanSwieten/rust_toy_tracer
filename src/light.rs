use super::types::{Direction, Position};

pub trait Light {
    fn sample(&self, position: &Position) -> Direction;
    fn intensity(&self) -> f32;
}

pub struct DirectionalLight {
    direction: Direction,
    intenstity: f32,
}

impl Light for DirectionalLight {
    fn sample(&self, _: &Position) -> Direction {
        self.direction
    }

    fn intensity(&self) -> f32 {
        self.intenstity
    }
}

pub struct SphericalLight {
    radius: f32,
    intensity: f32,
}
