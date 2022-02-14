use super::resources::Resources;
use super::types::*;
use super::vec::*;
pub trait Texture {
    fn uid(&self) -> usize;
    fn sample(&self, resources: &Resources, uv: &TextureCoordinate, position: &Position) -> Color;
}

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn new(color: &Color) -> Self {
        Self { color: *color }
    }
}

impl Texture for SolidColorTexture {
    fn sample(&self, _resources: &Resources, _: &TextureCoordinate, _: &Position) -> Color {
        self.color
    }

    fn uid(&self) -> usize {
        1
    }
}

pub struct CheckerTexture {
    even: u32,
    odd: u32,
    frequency: f32,
}

impl CheckerTexture {
    pub fn new(even: u32, odd: u32, frequency: f32) -> Self {
        Self {
            even: even,
            odd: odd,
            frequency,
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, resources: &Resources, uv: &TextureCoordinate, position: &Position) -> Color {
        let sines = (position.x() * self.frequency).sin()
            * (position.y() * self.frequency).sin()
            * (position.z() * self.frequency).sin();
        if sines < 0. {
            resources.texture(self.odd).sample(resources, uv, position)
        } else {
            resources.texture(self.even).sample(resources, uv, position)
        }
    }

    fn uid(&self) -> usize {
        2
    }
}
