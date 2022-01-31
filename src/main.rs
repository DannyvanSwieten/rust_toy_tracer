extern crate num_cpus;

pub mod acceleration_structure;
pub mod bounding_box;
pub mod cpu_tracer;
pub mod hittable;
pub mod intersection;
pub mod material;
pub mod rand;
pub mod ray;
pub mod raytracer;
pub mod scene;
pub mod types;
pub mod vec;
pub mod vec_add;
pub mod vec_div;
pub mod vec_mul;
pub mod vec_sub;

pub mod mat;
pub mod materials;

use acceleration_structure::*;
use cpu_tracer::*;
use hittable::*;
use material::*;
use materials::*;
use ray::*;
use raytracer::*;
use scene::*;
use std::sync::Arc;
use std::time::Instant;
use types::*;
use vec::*;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    fn new(color: &Color) -> Self {
        Self { color: *color }
    }
}

impl Texture for SolidColorTexture {
    fn sample(&self, _: &TextureCoordinate, _: &Position) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
    frequency: f32,
}

impl CheckerTexture {
    fn new(
        even: Arc<dyn Texture + Send + Sync>,
        odd: Arc<dyn Texture + Send + Sync>,
        frequency: f32,
    ) -> Self {
        Self {
            even,
            odd,
            frequency,
        }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: &TextureCoordinate, position: &Position) -> Color {
        let sines = (position.x() * self.frequency).sin()
            * (position.y() * self.frequency).sin()
            * (position.z() * self.frequency).sin();
        if sines < 0. {
            self.odd.sample(uv, position)
        } else {
            self.even.sample(uv, position)
        }
    }
}

pub struct CameraSettings {
    origin: Position,
    left_corner: Position,
    horizontal: Direction,
    vertical: Direction,
}

impl CameraSettings {
    fn new(origin: &Position, look_at: &Direction, aspect_ratio: f32, fov: f32) -> Self {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(&((*origin) - look_at));
        let up = Direction::from_values(&[0., 1., 0.]);
        let u = cross(&up, &w);
        let v = cross(&w, &u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let left_corner = *origin - horizontal / 2. - vertical / 2. - w;
        Self {
            origin: *origin,
            left_corner,
            horizontal,
            vertical,
        }
    }

    fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            &self.origin,
            &(self.left_corner + self.horizontal * u + self.vertical * v - self.origin),
        )
    }
}

struct RayGenerator {
    camera: CameraSettings,
}

impl RayGenerationShader<MyContext> for RayGenerator {
    fn generate(
        &self,
        ray_tracer: &dyn RayTracer<MyContext>,
        context: &MyContext,
        scene: &AccelerationStructure,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> Color {
        let mut color = Color::from_values(&[0., 0., 0.]);
        for _ in 0..context.spp {
            let mut coefficient = Color::from_values(&[1., 1., 1.]);
            let u = (x as f32 + rand::float()) / (width - 1) as f32;
            let v = (y as f32 + rand::float()) / (height - 1) as f32;
            let mut ray = self.camera.ray(u, 1. - v);
            for d in 0..context.max_depth {
                if let Some((instance_id, hit)) = ray_tracer.intersect(context, scene, &ray) {
                    let instance = scene.instance(instance_id as usize);
                    let geometry = scene.geometry(instance.geometry_index as usize);
                    let material_id = context.material_ids[instance.instance_id as usize];
                    let material = &context.materials[material_id as usize];
                    let normal = geometry.normal(&instance.transform, &hit);
                    let uv = geometry.uv(&instance.transform, &hit);

                    if let Some(bounce) =
                        material.wi(&hit.position, &hit.in_direction, &normal, &uv)
                    {
                        coefficient = coefficient * bounce.color;
                        let p = ray.at(hit.t);
                        ray = Ray::new(&(p + normal * 0.05), &bounce.out_dir);
                    } else {
                        coefficient = Color::from_values(&[0., 0., 0.]);
                        break;
                    }
                } else {
                    let d = 0.5 * ray.dir.y() + 1.;
                    let c = Color::from_values(&[1.0, 1.0, 1.0]) * (1.0 - d)
                        + Color::from_values(&[0.5, 0.7, 1.0]) * d;
                    coefficient *= c;
                    break;
                }

                // Russtion roullette
                if d > 3 && length(&coefficient) > rand::float() {
                    break;
                }
            }

            color = color + coefficient;
        }

        color = color / context.spp as f32;
        color
    }
}

struct MyContext {
    spp: u32,
    max_depth: u32,
    materials: Vec<Box<dyn Material + Send + Sync>>,
    material_ids: Vec<u32>,
}

fn main() {
    let width = 720;
    let height = 480;
    let camera = CameraSettings::new(
        &Position::from_values(&[2., 2., 13.]),
        &Direction::from_values(&[0., 0., 0.]),
        width as f32 / height as f32,
        45.,
    );
    let mut ctx = MyContext {
        spp: 16,
        max_depth: 8,
        materials: Vec::new(),
        material_ids: Vec::new(),
    };

    ctx.materials.push(Box::new(DiffuseMaterial::new(Arc::new(
        CheckerTexture::new(
            Arc::new(SolidColorTexture::new(&Color::from_values(&[1., 1., 1.]))),
            Arc::new(SolidColorTexture::new(&Color::from_values(&[0., 0., 0.]))),
            5.,
        ),
    ))));

    ctx.materials.push(Box::new(DiffuseMaterial::new(Arc::new(
        SolidColorTexture::new(&Color::from_values(&[1., 0., 1.])),
    ))));

    ctx.materials.push(Box::new(DiffuseMaterial::new(Arc::new(
        SolidColorTexture::new(&Color::from_values(&[1., 1., 1.])),
    ))));

    ctx.materials.push(Box::new(MirrorMaterial::new(Arc::new(
        SolidColorTexture::new(&Color::from_values(&[1., 1., 1.])),
    ))));

    let mut geometry: Vec<Arc<dyn Hittable + Send + Sync>> = Vec::new();

    let obj_file = "./assets/cube_rounded.obj";
    let (models, _) = tobj::load_obj(&obj_file, false).expect("Failed to load file");

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut tex_coords = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // positions.push(Position::from_values(&[-0.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.0, 0.5, 0.0]));

    // positions.push(Position::from_values(&[-0.5 + 1.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.5 + 1.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.0 + 1.5, 0.5, 0.0]));

    // positions.push(Position::from_values(&[-0.5 - 1.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.5 - 1.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.0 - 1.5, 0.5, 0.0]));

    // positions.push(Position::from_values(&[-0.5 - 2.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.5 - 2.5, -0.5, 0.0]));
    // positions.push(Position::from_values(&[0.0 - 2.5, 0.5, 0.0]));

    // indices.extend(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 11]);

    for (_, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        indices.extend(mesh.indices.iter());

        for v in 0..mesh.positions.len() / 3 {
            positions.push(Position::from_values(&[
                mesh.positions[3 * v],
                mesh.positions[3 * v + 1],
                mesh.positions[3 * v + 2],
            ]))
        }

        for n in 0..mesh.normals.len() / 3 {
            normals.push(Normal::from_values(&[
                mesh.normals[3 * n],
                mesh.normals[3 * n + 1],
                mesh.normals[3 * n + 2],
            ]))
        }

        for n in 0..mesh.texcoords.len() / 3 {
            tex_coords.push(TextureCoordinate::from_values(&[
                mesh.texcoords[3 * n],
                mesh.texcoords[3 * n + 1],
            ]))
        }
    }

    geometry.push(Arc::new(TriangleMesh::new(
        positions, normals, tex_coords, indices,
    )));

    let mut instances = Vec::new();
    // Floor
    instances.push(
        Instance::new(0, 0)
            .with_position(0., 0., 0.)
            .with_scale(1., 1., 1.),
    );
    // Checker
    ctx.material_ids.push(0);
    // Teapot
    instances.push(Instance::new(0, 1).with_position(0., 0., 0.));
    // Purple
    ctx.material_ids.push(2);

    for i in 2..50 {
        let x = rand::float_range(-5., 5.).floor();
        let y = rand::float_range(-5., 5.).floor();
        let z = rand::float_range(1., 5.).floor();
        let s = rand::float_range(0.25, 1.25);
        instances.push(
            Instance::new(0, i)
                .with_position(x, y, z)
                .with_scale(s, s, s),
        );
        ctx.material_ids.push(rand::int_range(0, 3));
    }

    let ac = AccelerationStructure::new(&geometry, &instances);
    let tracer = CPUTracer::new(Arc::new(RayGenerator { camera: camera }));

    let start = Instant::now();
    tracer.trace(&ctx, width, height, &ac);
    let duration = start.elapsed();

    println!("Time elapsed in trace() is: {:?}", duration);
}
