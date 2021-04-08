use super::acceleration_structure::*;
use super::intersection::*;
use super::ray::*;
use super::scene::*;
use super::types::*;

pub trait RayTracer<Context> {
    fn trace(&self, context: &Context, width: u32, height: u32, scene: &AccelerationStructure);
    fn intersect(&self, context: &Context, scene: &AccelerationStructure, ray: &Ray) -> Option<Intersection>;
}

pub trait RayGenerationShader<Context> {
    fn generate(
        &self,
        ray_tracer: &dyn RayTracer<Context>,
        context: &Context,
        scene: &AccelerationStructure,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> Color;
}

pub trait ClosestHitShader<Context> {
    fn hit(&self, ctx: &Context, intersection: &Intersection);
}
