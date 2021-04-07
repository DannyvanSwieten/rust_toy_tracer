pub mod bounding_box;
pub mod hittable;
pub mod intersection;
pub mod material;
pub mod rand_float;
pub mod ray;
pub mod raytracer;
pub mod scene;
pub mod types;

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crossbeam::thread;
use hittable::*;
use intersection::*;
use material::*;
use rand_float::*;
use ray::*;
use raytracer::*;
use scene::*;
use types::*;

use glm;
use glm::builtin::*;

use image; // 0.23.14
use image::imageops::*;
use image::{GenericImage, GenericImageView, Rgb, RgbImage};

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
    even: Box<dyn Texture + Send + Sync>,
    odd: Box<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    fn new(even: Box<dyn Texture + Send + Sync>, odd: Box<dyn Texture + Send + Sync>) -> Self {
        Self { even, odd }
    }
}

impl Texture for CheckerTexture {
    fn sample(&self, uv: &TextureCoordinate, position: &Position) -> Color {
        let sines = sin(position.x * 10.) * sin(position.y * 10.) * sin(position.z * 10.);
        if sines < 0. {
            self.odd.sample(uv, position)
        } else {
            self.even.sample(uv, position)
        }
    }
}

pub struct DiffuseMaterial {
    albedo: Box<dyn Texture + Send + Sync>,
}

impl DiffuseMaterial {
    fn new(albedo: Box<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

impl Material for DiffuseMaterial {
    fn brdf(&self, surface: &Intersection) -> Option<Bounce> {
        Some(Bounce {
            color: self.albedo.sample(&surface.uv, &surface.position) / std::f32::consts::PI,
            out_dir: surface.normal + rand_sphere(),
        })
    }
    fn pdf(&self, _: &Intersection) -> f32 {
        1.
    }
}

pub struct MirrorMaterial {
    albedo: Box<dyn Texture + Send + Sync>,
}

impl Material for MirrorMaterial {
    fn brdf(&self, surface: &Intersection) -> Option<Bounce> {
        Some(Bounce {
            color: self.albedo.sample(&surface.uv, &surface.position),
            out_dir: reflect(surface.in_direction, surface.normal),
        })
    }
    fn pdf(&self, _: &Intersection) -> f32 {
        1.
    }
}

impl MirrorMaterial {
    fn new(albedo: Box<dyn Texture + Send + Sync>) -> Self {
        Self { albedo }
    }
}

pub struct CameraSettings {
    origin: Position,
    left_corner: Position,
    horizontal: Direction,
    vertical: Direction,
    u: Direction,
    v: Direction,
    w: Direction,
}

impl CameraSettings {
    fn new(origin: &Position, look_at: &Direction, aspect_ratio: f32, fov: f32) -> Self {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(*origin - *look_at);
        let u = cross(Direction::new(0., 1., 0.), w);
        let v = cross(w, u);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let left_corner = *origin - horizontal / 2. - vertical / 2. - w;
        Self {
            origin: *origin,
            left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
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
    ray_generation_shader: Box<dyn RayGenerationShader<Context> + Send + Sync>,
}

impl<Context> CPUTracer<Context> {
    fn new(ray_generation_shader: Box<dyn RayGenerationShader<Context> + Send + Sync>) -> Self {
        Self {
            ray_generation_shader,
        }
    }
}

impl<Context: Send + Sync> RayTracer<Context> for CPUTracer<Context> {
    fn trace(&self, context: &Context, width: u32, height: u32, scene: &Scene) {
        let thread_count: u32 = 64;
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
                    let r = (color.x.sqrt() * 255.) as u8;
                    let g = (color.y.sqrt() * 255.) as u8;
                    let b = (color.z.sqrt() * 255.) as u8;
                    image.put_pixel(x, y, Rgb([r, g, b]))
                }
            }
        }

        image.save("output.png");
    }

    fn intersect(&self, _: &Context, scene: &Scene, ray: &Ray) -> Option<Intersection> {
        scene.intersect(ray, 0.01, 1000.)
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
        scene: &Scene,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> Color {
        let mut color = Color::new(0., 0., 0.);
        for _ in 0..context.spp {
            let mut coefficient = Color::new(1., 1., 1.);
            let u = (x as f32 + rand_float()) / (width - 1) as f32;
            let v = (y as f32 + rand_float()) / (height - 1) as f32;
            let mut ray = self.camera.ray(u, 1. - v);
            for _ in 0..context.max_depth {
                if let Some(hit) = ray_tracer.intersect(context, scene, &ray) {
                    if let Some(bounce) = context.materials[hit.material_id as usize].brdf(&hit) {
                        coefficient = coefficient * bounce.color;
                        let p = ray.at(hit.t);
                        ray = Ray::new(&p, &bounce.out_dir);
                    } else {
                        coefficient = Color::new(0., 0., 0.);
                        break;
                    }
                } else {
                    let d = 0.5 * ray.dir.y + 1.;
                    let c = Color::new(1.0, 1.0, 1.0) * (1.0 - d) + Color::new(0.5, 0.7, 1.0) * d;
                    coefficient = coefficient * c;
                    break;
                }
            }

            color = color + coefficient;
        }

        color = color / context.spp as f32;
        color

        //context.output_image.put_pixel(x, y, Rgb([r, g, b]));
    }
}

struct MyContext {
    spp: u32,
    max_depth: u32,
    materials: Vec<Box<dyn Material + Send + Sync>>,
}

fn main() {
    let width = 1280;
    let height = 720;
    let mut scene = Scene::new();
    let camera = CameraSettings::new(
        &Position::new(0., 2., 13.),
        &Direction::new(0., 2., 0.),
        16. / 9.,
        65.,
    );
    let mut ctx = MyContext {
        spp: 64,
        max_depth: 16,
        materials: Vec::new(),
    };

    ctx.materials.push(Box::new(DiffuseMaterial::new(Box::new(
        CheckerTexture::new(
            Box::new(SolidColorTexture::new(&Color::new(1., 1., 1.))),
            Box::new(SolidColorTexture::new(&Color::new(0., 0., 0.))),
        ),
    ))));

    ctx.materials.push(Box::new(DiffuseMaterial::new(Box::new(
        SolidColorTexture::new(&Color::new(1., 0., 1.)),
    ))));

    ctx.materials.push(Box::new(DiffuseMaterial::new(Box::new(
        SolidColorTexture::new(&Color::new(1., 1., 1.)),
    ))));

    ctx.materials.push(Box::new(MirrorMaterial::new(Box::new(
        SolidColorTexture::new(&Color::new(0., 1., 1.)),
    ))));

    // Floor
    scene.add_hittable(Box::new(Sphere::new(
        1000.,
        &Position::new(0., -1000., 0.),
        0,
    )));

    for _ in 0..30 {
        scene.add_hittable(Box::new(Sphere::new(
            rand_range(0.5, 1.25),
            &Position::new(
                rand_range(-10., 10.),
                rand_range(1., 10.),
                rand_range(2., 10.),
            ),
            rand_range(0., 4.) as u32,
        )));
    }
    scene.add_hittable(Box::new(Sphere::new(1., &Position::new(0., 1., 0.), 1)));
    let tracer = CPUTracer::new(Box::new(RayGenerator { camera: camera }));

    tracer.trace(&ctx, width, height, &scene);
}
