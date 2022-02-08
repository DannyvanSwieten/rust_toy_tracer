use super::acceleration_structure::*;
use super::hittable::Hittable;
use super::intersection::*;
use super::ray::*;
use super::raytracer::*;
use super::resources::Resources;
use super::types::Color;
use super::vec::{XAccessor, YAccessor, ZAccessor};
use crossbeam::thread;
use image::*;
use std::sync::mpsc::{channel, Receiver, Sender};
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
        resources: &Resources,
    ) {
        let thread_count = num_cpus::get() as u32;
        let mut image = RgbImage::new(width, height);
        let (tx, rx): (
            Sender<Vec<(u32, u32, Color)>>,
            Receiver<Vec<(u32, u32, Color)>>,
        ) = channel();

        for row in (0..height).step_by(thread_count as usize) {
            if row >= height {
                break;
            }

            let thread_tx = tx.clone();

            thread::scope(|s| {
                for t in 0..thread_count {
                    if row + t >= height {
                        break;
                    }
                    let thread_tx = thread_tx.clone();

                    s.spawn(move |_| {
                        let mut row_vector = Vec::with_capacity(width as usize);
                        for x in 0..width {
                            let color = self.ray_generation_shader.generate(
                                self,
                                scene,
                                resources,
                                spp,
                                max_depth,
                                width,
                                height,
                                x,
                                row + t,
                            );

                            row_vector.push((x, row + t, color));
                        }

                        thread_tx.send(row_vector).unwrap();
                        let progress = (row + t) as f32 / height as f32;
                        println!("Progress: {} Row: {}", progress * 100., row + t);
                    });
                }
            })
            .unwrap();

            for row in rx.try_iter() {
                for (x, y, color) in row {
                    let c = color / (Color::from_values(&[1.0, 1.0, 1.0]) + color);
                    let r = (c.x().sqrt() * 255.) as u8;
                    let g = (c.y().sqrt() * 255.) as u8;
                    let b = (c.z().sqrt() * 255.) as u8;
                    image.put_pixel(x, y, Rgb([r, g, b]))
                }
            }
        }

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
                if let Some(intersection) = resources
                    .hittable(instance.geometry_index as usize)
                    .intersect(&instance.transform, ray, 0.01, 1000.)
                {
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
