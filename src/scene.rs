use super::bounding_box::*;
use super::hittable::*;
use super::intersection::*;
use super::ray::*;
use super::types::*;
use std::sync::Arc;

#[derive(Copy, Clone)]
pub struct Instance {
    pub geometry_index: u32,
    pub instance_id: u32,
    pub hit_shader_id: u32,
    //pub transform: Transform,
}

impl Instance {
    // fn new(geometry_index: u32, instance_id: u32) -> Self {
    //     Self{
    //         geometry_index,
    //         instance_id,
    //         hit_shader_id: 0,
    //         transform: Transform::new(
    //             Vec4::new(1., 0., 0., 0.),
    //             Vec4::new(0., 1., 0., 0.),
    //             Vec4::new(0., 0., 1., 1.),
    //         )
    //     }
    // }
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
