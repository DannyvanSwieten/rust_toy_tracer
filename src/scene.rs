use super::bounding_box::*;
use super::hittable::*;
use super::intersection::*;
use super::ray::*;
use super::types::*;
use glm::Matrix4x3;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct Instance {
    pub object_id: u32,
    pub hit_shader_id: u32,
    pub transform: Transform,
}

pub struct Scene {
    hittables: Vec<Arc<dyn Hittable + Send + Sync>>,
    instances: Vec<Instance>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            hittables: Vec::new(),
            instances: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, t: Arc<dyn Hittable + Send + Sync>) -> usize {
        self.hittables.push(t);
        self.hittables.len() - 1
    }

    pub fn hittables(&self) -> &Vec<Arc<dyn Hittable + Send + Sync>> {
        &self.hittables
    }
}
