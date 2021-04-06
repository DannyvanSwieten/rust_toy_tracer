use super::intersection::*;
use super::types::*;

pub struct Bounce {
    pub color: Color,
    pub out_dir: Direction,
}

pub trait Material {
    fn brdf(&self, surface: &Intersection) -> Option<Bounce>;
    fn pdf(&self, surface: &Intersection) -> f32;
}

pub trait Texture {
    fn sample(&self, uv: &TextureCoordinate, position: &Position) -> Color;
}
