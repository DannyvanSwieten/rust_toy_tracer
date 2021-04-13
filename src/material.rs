use super::intersection::*;
use super::types::*;

pub struct Bounce {
    pub color: Color,
    pub out_dir: Direction,
}

pub trait Material {
    fn wi(&self, position: &Position, wo: &Direction, normal: &Direction, uv: &TextureCoordinate) -> Option<Bounce>;
    fn pdf(&self, surface: &Intersection) -> f32;
}

pub trait Texture {
    fn sample(&self, uv: &TextureCoordinate, position: &Position) -> Color;
}
