use super::bounding_box::*;
use super::hittable::*;
use super::intersection::*;
use super::ray::*;
use super::types::*;
use glm::Matrix4x3;
use std::sync::Arc;

pub struct Instance {
    object_id: u32,
    hit_shader_id: u32,
    transform: Matrix4x3<f32>,
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
    fn bounding_box(&self) -> Option<BoundingBox> {
        if self.hittables.len() == 0 {
            return None;
        }

        let tmp = BoundingBox::new(&Position::new(0., 0., 0.), &Position::new(0., 0., 0.));
        let mut output = BoundingBox::new(&Position::new(0., 0., 0.), &Position::new(0., 0., 0.));
        let mut first_box = true;

        for hittable in self.hittables.iter() {
            if hittable.bounding_box().is_none() {
                return None;
            }

            if first_box {
                output = tmp;
                first_box = false
            } else {
                output = BoundingBox::surrounding_box(&output, &tmp);
            }
        }

        Some(output)
    }
}
