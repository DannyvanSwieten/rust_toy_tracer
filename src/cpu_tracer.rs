use crate::light::Lights;

use super::acceleration_structure::*;
use super::intersection::*;
use super::ray::*;
use super::raytracer::*;
use super::resources::Resources;
use super::types::Color;
use super::vec::{XAccessor, YAccessor, ZAccessor};

use image::*;
use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
pub struct CPUTracer {
    ray_generation_shader: Box<dyn RayGenerationShader>,
}

impl CPUTracer {
    pub fn new<T>(ray_generation_shader: T) -> Self
    where
        T: RayGenerationShader + 'static,
    {
        Self {
            ray_generation_shader: Box::new(ray_generation_shader),
        }
    }
}

unsafe impl Send for CPUTracer {}
unsafe impl Sync for CPUTracer {}

impl RayTracer for CPUTracer {
    fn trace(
        &self,
        spp: u32,
        max_depth: u32,
        width: u32,
        height: u32,
        scene: &TopLevelAccelerationStructure,
        lights: &Lights,
        resources: &Resources,
    ) {
        let mut image = RgbImage::new(width, height);
        (0..height).into_iter().for_each(|y| {
            let row: Vec<Rgb<u8>> = (0..width)
                .into_par_iter()
                .map(|x| {
                    let color = self.ray_generation_shader.generate(
                        self, scene, lights, resources, spp, max_depth, width, height, x, y,
                    );
                    let c = color / (Color::from_values([1.0, 1.0, 1.0]) + color);
                    let r = (c.x().sqrt() * 255.) as u8;
                    let g = (c.y().sqrt() * 255.) as u8;
                    let b = (c.z().sqrt() * 255.) as u8;
                    Rgb([r, g, b])
                })
                .collect();
            for i in 0..width {
                image.put_pixel(i, y, row[i as usize])
            }
        });

        image.save("output.png").expect("Write to image failed");
    }

    fn intersect(
        &self,
        ray: &Ray,
        scene: &TopLevelAccelerationStructure,
        resources: &Resources,
    ) -> Option<(u32, Intersection)> {
        let results = scene.intersect_instance(ray, 0.01, 1000.);
        if results.len() > 0 {
            let mut closest = None;
            let mut t: f32 = 1001.;
            for id in results.iter() {
                let instance = scene.instance(*id as usize);
                if let Some(intersection) = resources.hittable(instance.geometry_index).intersect(
                    &instance.transform,
                    ray,
                    instance.cull,
                    0.01,
                    1000.,
                ) {
                    if intersection.t < t {
                        t = intersection.t;
                        closest = Some((instance.instance_id, intersection));
                    }
                }
            }

            return closest;
        }

        None
    }
}
