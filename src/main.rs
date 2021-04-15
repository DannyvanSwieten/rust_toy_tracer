extern crate num_cpus;

pub mod acceleration_structure;
pub mod bounding_box;
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

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

use acceleration_structure::*;
use crossbeam::thread;
use hittable::*;
use intersection::*;
use material::*;
use ray::*;
use raytracer::*;
use scene::*;
use types::*;
use vec::*;

use image; // 0.23.14
use image::{Rgb, RgbImage};

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

pub struct DiffuseMaterial {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseMaterial {
    fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

impl Material for DiffuseMaterial {
    fn wi(
        &self,
        position: &Position,
        wo: &Direction,
        normal: &Direction,
        uv: &TextureCoordinate,
    ) -> Option<Bounce> {
        Some(Bounce {
            color: self.albedo.sample(&uv, &position) / std::f32::consts::PI,
            out_dir: *normal + rand::sphere(),
        })
    }
    fn pdf(&self, _: &Intersection) -> f32 {
        1.
    }
}

pub struct MirrorMaterial {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Material for MirrorMaterial {
    fn wi(
        &self,
        position: &Position,
        wo: &Direction,
        normal: &Direction,
        uv: &TextureCoordinate,
    ) -> Option<Bounce> {
        Some(Bounce {
            color: self.albedo.sample(&uv, &position),
            out_dir: reflect(&wo, &normal),
        })
    }
    fn pdf(&self, _: &Intersection) -> f32 {
        1.
    }
}

impl MirrorMaterial {
    fn new(albedo: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
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

pub struct CPUTracer<Context> {
    ray_generation_shader: Arc<dyn RayGenerationShader<Context> + Send + Sync>,
}

impl<Context> CPUTracer<Context> {
    fn new(ray_generation_shader: Arc<dyn RayGenerationShader<Context> + Send + Sync>) -> Self {
        Self {
            ray_generation_shader,
        }
    }
}

impl<Context: Send + Sync> RayTracer<Context> for CPUTracer<Context> {
    fn trace(&self, context: &Context, width: u32, height: u32, scene: &AccelerationStructure) {
        let thread_count = num_cpus::get() as u32;
        let mut image = RgbImage::new(width, height);
        let (tx, rx): (
            Sender<Vec<(u32, u32, Color)>>,
            Receiver<Vec<(u32, u32, Color)>>,
        ) = mpsc::channel();

        for row in (0..height).step_by(thread_count as usize) {
            if row >= height {
                break;
            }

            let thread_tx = tx.clone();

            //let slice = acc[(row * width) as usize..(row * width + width) as usize];
            let scope = thread::scope(move |s| {
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
                                context,
                                scene,
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
                    let r = (color.x().sqrt() * 255.) as u8;
                    let g = (color.y().sqrt() * 255.) as u8;
                    let b = (color.z().sqrt() * 255.) as u8;
                    image.put_pixel(x, y, Rgb([r, g, b]))
                }
            }
        }

        image.save("output.png");
    }

    fn intersect(
        &self,
        _: &Context,
        scene: &AccelerationStructure,
        ray: &Ray,
    ) -> Option<(u32, Intersection)> {
        let results = scene.intersect_instance(ray, 0.01, 1000.);
        if results.len() > 0 {
            let mut closest = None;
            let mut t: f32 = 1001.;
            for id in results.iter() {
                let instance = scene.instance(*id as usize);
                if let Some(intersection) = scene
                    .geometry(instance.geometry_index as usize)
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
    let width = 1920;
    let height = 1080;
    let camera = CameraSettings::new(
        &Position::from_values(&[0., 2., 13.]),
        &Direction::from_values(&[0., 0., 0.]),
        16. / 9.,
        65.,
    );
    let mut ctx = MyContext {
        spp: 32,
        max_depth: 16,
        materials: Vec::new(),
        material_ids: Vec::new(),
    };

    ctx.materials.push(Box::new(DiffuseMaterial::new(Arc::new(
        CheckerTexture::new(
            Arc::new(SolidColorTexture::new(&Color::from_values(&[1., 1., 1.]))),
            Arc::new(SolidColorTexture::new(&Color::from_values(&[0., 0., 0.]))),
            3.,
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

    geometry.push(Arc::new(Sphere::new(
        1.,
        &Position::from_values(&[0., 0., 0.]),
        0,
    )));

    geometry.push(Arc::new(TriangleMesh::new(
        positions, normals, tex_coords, indices,
    )));

    let mut instances = Vec::new();
    // Floor
    instances.push(
        Instance::new(0, 0)
            .with_position(0., -1001., 0.)
            .with_scale(1000., 1000., 1000.),
    );
    // Checker
    ctx.material_ids.push(0);
    // Teapot
    instances.push(Instance::new(1, 1).with_position(0., 0., 0.));
    // Purple
    ctx.material_ids.push(2);

    for i in 2..50 {
        let x = rand::float_range(-10., 10.).floor();
        let y = rand::float_range(0., 10.).floor();
        let z = rand::float_range(1., 5.).floor();
        let s = rand::float_range(0.25, 1.25);
        instances.push(
            Instance::new(rand::int_range(0, 2), i)
                .with_position(x, y, z)
                .with_scale(s, s, s),
        );
        ctx.material_ids.push(rand::int_range(0, 3));
    }

    let ac = AccelerationStructure::new(&geometry, &instances);
    let tracer = CPUTracer::new(Arc::new(RayGenerator { camera: camera }));

    tracer.trace(&ctx, width, height, &ac);
}
