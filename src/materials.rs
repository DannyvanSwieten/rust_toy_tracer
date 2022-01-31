use super::intersection::*;
use super::material::*;
use super::rand;
use super::types::*;
use super::vec::*;
use std::sync::Arc;

pub struct DiffuseMaterial {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseMaterial {
    pub fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

impl Material for DiffuseMaterial {
    fn wi(
        &self,
        position: &Position,
        _wo: &Direction,
        normal: &Direction,
        uv: &TextureCoordinate,
    ) -> Option<Bounce> {
        Some(Bounce {
            color: self.albedo.sample(&uv, &position) / std::f32::consts::PI,
            out_dir: *normal + rand::sphere(),
        })
    }
    fn pdf(&self, _: &Intersection) -> f32 {
        1.
    }
}

pub struct MirrorMaterial {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Material for MirrorMaterial {
    fn wi(
        &self,
        position: &Position,
        wo: &Direction,
        normal: &Direction,
        uv: &TextureCoordinate,
    ) -> Option<Bounce> {
        Some(Bounce {
            color: self.albedo.sample(&uv, &position),
            out_dir: reflect(&wo, &normal),
        })
    }
    fn pdf(&self, _: &Intersection) -> f32 {
        1.
    }
}

impl MirrorMaterial {
    pub fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}
