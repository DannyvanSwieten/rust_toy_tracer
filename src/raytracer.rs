use super::intersection::*;
use super::ray::*;
use super::scene::*;
use super::types::*;

pub trait RayTracer<Context> {
    fn trace(&self, context: &Context, width: u32, height: u32, scene: &Scene);
    fn intersect(&self, context: &Context, scene: &Scene, ray: &Ray) -> Option<Intersection>;
}

pub trait RayGenerationShader<Context> {
    fn generate(
        &self,
        ray_tracer: &'static dyn RayTracer<Context>,
        context: &'static Context,
        scene: &'static Scene,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> Color;
}

pub trait ClosestHitShader<Context> {
    fn hit(&self, ctx: &Context, intersection: &Intersection);
}
