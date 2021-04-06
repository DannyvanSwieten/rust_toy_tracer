use super::hittable::*;
use super::intersection::*;
use super::ray::*;
use glm::Matrix4x3;

pub struct Instance {
    object_id: u32,
    hit_shader_id: u32,
    transform: Matrix4x3<f32>,
}

pub struct Scene {
    hittables: Vec<Box<dyn Hittable + Send + Sync>>,
    instances: Vec<Instance>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            hittables: Vec::new(),
            instances: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, t: Box<dyn Hittable + Send + Sync>) -> usize {
        self.hittables.push(t);
        self.hittables.len() - 1
    }
}

impl Hittable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut t = f32::MAX;
        let mut intersection = None;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.intersect(ray, t_min, t_max) {
                if hit.t < t {
                    t = hit.t;
                    intersection = Some(hit);
                }
            }
        }

        return intersection;
    }
}
