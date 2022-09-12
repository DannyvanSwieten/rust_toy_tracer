use super::rand;
use crate::acceleration_structure::TopLevelAccelerationStructure;
use crate::default_camera::DefaultCamera;
use crate::light::Lights;
use crate::material::HitRecord;
use crate::ray::Ray;
use crate::raytracer::{RayGenerationShader, RayTracer};
use crate::resources::Resources;
use crate::types::*;
use crate::vec::{dot, length, YAccessor};

pub struct RayGenerator {
    pub camera: DefaultCamera,
}

impl RayGenerationShader for RayGenerator {
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
    ) -> Color {
        let mut color = Color::new();
        for _ in 0..spp {
            let mut coefficient = Color::from_values([1., 1., 1.]);
            let u = (x as f32 + rand::float()) / (width - 1) as f32;
            let v = (y as f32 + rand::float()) / (height - 1) as f32;
            let mut ray = self.camera.ray(u, 1. - v);
            for d in 0..max_depth {
                if let Some((instance_id, hit)) =
                    ray_tracer.intersect(&ray, scene, resources, 0.001, 1000.0)
                {
                    let instance = scene.instance(instance_id as usize);
                    let geometry = resources.hittable(instance.geometry_index);
                    let material_id = instance.material_id;
                    let material = resources.material(material_id);

                    let mut hit_record = HitRecord::default();
                    hit_record.instance_id = instance_id;
                    hit_record.intersection = hit;
                    hit_record.uv = geometry.uv(&instance.transform, &hit_record.intersection);
                    hit_record.normal =
                        geometry.normal(&instance.transform, &hit_record.intersection);
                    hit_record.front_facing =
                        dot(&hit_record.normal, &hit_record.ray_direction()) < 0.0;
                    let bounce = material.evaluate(resources, &hit_record);

                    let mut indirect_light = bounce.color;

                    let mut direct_light = Color::new();
                    // for light in lights.data() {
                    //     let ray_dir = light.sample(&Position::default());
                    //     let ray_origin = hit_record.position() + ray_dir * 0.05;
                    //     let shadow_ray = Ray::new(&ray_origin, &ray_dir);
                    //     match ray_tracer.intersect(&shadow_ray, scene, resources) {
                    //         None => {
                    //             let direct_color = material.evaluate(resources, &hit_record);
                    //             direct_light += &(light.color() * direct_color);
                    //         }
                    //         _ => (),
                    //     }
                    // }
                    coefficient *= indirect_light + direct_light;
                    coefficient = coefficient + material.emit(resources, &hit_record);
                    ray = Ray::new(&(hit_record.position()/*+ bounce.wi * 0.05*/), &bounce.wi)
                } else {
                    let di = 0.5 * ray.dir.y() + 1.;
                    let c = Color::from_values([1.0, 1.0, 1.0]) * (1.0 - di)
                        + Color::from_values([0.5, 0.7, 1.0]) * di;
                    coefficient *= c;
                    break;
                }

                // Russion roullette
                if d > 3 && length(&coefficient) < rand::float() {
                    break;
                }
            }

            color = color + coefficient;
        }

        color = color / spp as f32;
        color
    }
}
