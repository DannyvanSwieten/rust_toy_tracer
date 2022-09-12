use crate::camera::Camera;
use crate::light::Light;
use crate::light::Lights;

use super::acceleration_structure::*;
use super::intersection::*;
use super::ray::*;
use super::resources::Resources;
use super::types::*;

pub trait RayTracer {
    fn trace(
        &self,
        spp: u32,
        max_depth: u32,
        width: u32,
        height: u32,
        scene: &TopLevelAccelerationStructure,
        lights: &Lights,
        resources: &Resources,
    );
    fn intersect(
        &self,
        ray: &Ray,
        scene: &TopLevelAccelerationStructure,
        resources: &Resources,
        t_min: f32,
        t_max: f32,
    ) -> Option<(u32, Intersection)>;
}

pub trait RayGenerationShader {
    fn generate(
        &self,
        ray_tracer: &dyn RayTracer,
        scene: &TopLevelAccelerationStructure,
        lights: &Lights,
        resources: &Resources,
        spp: u32,
        max_depth: u32,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> Color;
}

pub trait ClosestHitShader {
    fn hit(
        &self,
        scene: &TopLevelAccelerationStructure,
        intersection: &Intersection,
        object_to_world: &Transform,
    );
}
