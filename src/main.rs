extern crate num_cpus;

pub mod acceleration_structure;
pub mod bounding_box;
pub mod brdf;
pub mod cpu_tracer;
pub mod hittable;
pub mod intersection;
pub mod light;
pub mod mat;
pub mod material;
pub mod materials;
pub mod onb;
pub mod rand;
pub mod ray;
pub mod raytracer;
pub mod resources;
pub mod scene;
pub mod texture;
pub mod types;
pub mod vec;
pub mod vec_add;
pub mod vec_div;
pub mod vec_mul;
pub mod vec_sub;

use acceleration_structure::*;
use cpu_tracer::*;
use hittable::*;
use material::*;
use materials::*;
use ray::*;
use raytracer::*;
use resources::Resources;
use scene::*;
use texture::*;
use types::*;
use vec::*;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}

pub struct CameraSettings {
    origin: Position,
    left_corner: Position,
    horizontal: Direction,
    vertical: Direction,
    u: Direction,
    v: Direction,
    w: Direction,
    lens_radius: f32,
}

impl CameraSettings {
    fn new(
        origin: &Position,
        look_at: &Direction,
        aspect_ratio: f32,
        fov: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(&((*origin) - look_at));
        let up = Direction::from_values(&[0., 1., 0.]);
        let u = cross(&up, &w);
        let v = cross(&w, &u);

        let horizontal = focus_distance * u * viewport_width;
        let vertical = focus_distance * v * viewport_height;
        let left_corner = *origin - horizontal / 2. - vertical / 2. - focus_distance * w;
        Self {
            origin: *origin,
            left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = rand::disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            &(offset + self.origin),
            &normalize(
                &(self.left_corner + self.horizontal * s + self.vertical * t
                    - self.origin
                    - offset),
            ),
        )
    }
}

struct RayGenerator {
    camera: CameraSettings,
}

impl RayGenerationShader for RayGenerator {
    fn generate(
        &self,
        ray_tracer: &dyn RayTracer,
        scene: &TopLevelAccelerationStructure,
        resources: &Resources,
        spp: u32,
        max_depth: u32,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> Color {
        let mut color = Color::from_values(&[0., 0., 0.]);
        for _ in 0..spp {
            let mut coefficient = Color::from_values(&[1., 1., 1.]);
            let u = (x as f32 + rand::float()) / (width - 1) as f32;
            let v = (y as f32 + rand::float()) / (height - 1) as f32;
            let mut ray = self.camera.ray(u, 1. - v);
            let mut results = Vec::with_capacity(max_depth as _);
            for d in 0..max_depth {
                if let Some((instance_id, hit)) = ray_tracer.intersect(&ray, scene, resources) {
                    let instance = scene.instance(instance_id as usize);
                    let geometry = resources.hittable(instance.geometry_index as usize);
                    let material_id = instance.material_id as usize;
                    let material = resources.material(material_id as usize);

                    let mut hit_record = HitRecord::default();
                    hit_record.instance_id = instance_id;
                    hit_record.intersection = hit;
                    hit_record.uv = geometry.uv(&instance.transform, &hit_record.intersection);
                    hit_record.normal =
                        geometry.normal(&instance.transform, &hit_record.intersection);
                    hit_record.front_facing =
                        dot(&hit_record.normal, &hit_record.ray_direction()) < 0.0;
                    let bounce = material.scatter(resources, &hit_record);
                    coefficient *= bounce.color;
                    ray = bounce.ray;
                    hit_record.bounce = bounce;
                    results.push(hit_record);
                } else {
                    let d = 0.5 * ray.dir.y() + 1.;
                    let c = Color::from_values(&[1.0, 1.0, 1.0]) * (1.0 - d)
                        + Color::from_values(&[0.5, 0.7, 1.0]) * d;
                    coefficient = c;
                    break;
                }

                // Russion roullette
                if d > 3 && length(&coefficient) < rand::float() {
                    break;
                }
            }

            while let Some(hit) = results.pop() {
                let c = coefficient / hit.bounce.pdf;
                coefficient = c * hit.bounce.color;
            }

            color = color + coefficient;
        }

        color = color / spp as f32;
        color
    }
}

fn main() {
    let obj_file = "./assets/teapot.obj";
    let (models, _) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to load file");

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut tex_coords = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        indices.extend(mesh.indices.iter());

        for v in (0..mesh.positions.len()).step_by(3) {
            positions.push(Position::from_values(&[
                mesh.positions[v],
                mesh.positions[v + 1],
                mesh.positions[v + 2],
            ]))
        }

        for n in (0..mesh.normals.len()).step_by(3) {
            normals.push(Normal::from_values(&[
                mesh.normals[n],
                mesh.normals[n + 1],
                mesh.normals[n + 2],
            ]))
        }

        for t in (0..mesh.texcoords.len()).step_by(2) {
            tex_coords.push(TextureCoordinate::from_values(&[
                mesh.texcoords[t],
                mesh.texcoords[t + 1],
            ]))
        }
    }

    let mut resources = Resources::default();
    resources.add_hittable(Sphere::new(1.0, &Position::default()));
    resources.add_hittable(TriangleMesh::new(positions, normals, tex_coords, indices));

    resources.add_texture(SolidColorTexture::new(&Color::from_values(&[1., 1., 1.])));
    resources.add_texture(SolidColorTexture::new(&Color::from_values(&[0., 0., 0.])));
    resources.add_texture(SolidColorTexture::new(&Color::from_values(&[
        1., 0.5, 0.25,
    ])));
    resources.add_texture(CheckerTexture::new(0, 1, 3.0));

    resources.add_material(DiffuseMaterial::new(3));
    resources.add_material(DiffuseMaterial::new(2));
    resources.add_material(MirrorMaterial::new(0));
    resources.add_material(TranslucentMaterial::new(0, 1.5));
    resources.add_material(PBRMaterial::new());

    let mut instances = Vec::new();
    // Floor
    instances.push(
        Instance::new(0, 0, 0)
            .with_position(0., -100., 0.)
            .with_scale(100., 100., 100.),
    );

    instances.push(Instance::new(1, 1, 1).with_position(0., 0., -2.));
    instances.push(Instance::new(1, 2, 2).with_position(-5., 0., 0.));
    instances.push(Instance::new(1, 3, 3).with_position(5., 0., 2.));
    instances.push(Instance::new(1, 4, 4).with_position(10., 0., 4.));

    let ac = TopLevelAccelerationStructure::new(&resources.hittables(), &instances);
    let width = 720;
    let height = 480;
    let origin = &Position::from_values(&[-3., 4., 15.]);
    let look_at = &Direction::from_values(&[3., 0., 0.]);
    let camera = CameraSettings::new(
        &origin,
        &look_at,
        width as f32 / height as f32,
        45.,
        0.5,
        distance(&look_at, &origin),
    );

    let tracer = CPUTracer::new(RayGenerator { camera: camera });
    tracer.trace(128, 16, width, height, &ac, &resources);
}
