use super::ray::*;
use super::types::*;
#[derive(Default)]
pub struct Intersection {
    pub ray: Ray,
    pub t: f32,
    pub primitive_id: u32,
    pub barycentrics: Barycentrics,
}

impl Intersection {
    pub fn new(ray: &Ray, t: f32, primitive_id: u32, barycentrics: &Barycentrics) -> Self {
        Self {
            ray: *ray,
            t,
            primitive_id,
            barycentrics: *barycentrics,
        }
    }
}
