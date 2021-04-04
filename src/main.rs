use std::thread;

use glm;
use glm::builtin::*;
use glm::Matrix4x3;
use glm::Vector2;
use glm::Vector3;

use rand::Rng;

use image; // 0.23.14
use image::{Rgb, RgbImage};

type Color = Vector3<f32>;
type Normal = Vector3<f32>;
type Position = Vector3<f32>;
type Direction = Vector3<f32>;
type Barycentrics = Vector2<f32>;
type FragCoord = Vector2<u32>;
type Size2D = Vector2<u32>;
type TextureCoordinate = Vector2<f32>;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Position,
    dir: Direction,
}

impl Ray {
    fn new(origin: &Position, direction: &Direction) -> Self {
        return Self {
            origin: *origin,
            dir: *direction,
        };
    }

    fn origin(&self) -> &Position {
        &self.origin
    }

    fn direction(&self) -> &Direction {
        &self.dir
    }

    fn at(&self, t: f32) -> Position {
        self.origin + self.dir * t
    }
}

pub struct Intersection {
    position: Position,
    in_direction: Direction,
    t: f32,
    normal: Normal,
    uv: TextureCoordinate,
    object_id: u32,
    instance_id: u32,
    primitive_id: u32,
    material_id: u32,
    barycentrics: Barycentrics,
}

impl Intersection {
    pub fn new(
        position: &Position,
        in_direction: &Direction,
        t: f32,
        normal: &Normal,
        uv: &TextureCoordinate,
        object_id: u32,
        instance_id: u32,
        primitive_id: u32,
        material_id: u32,
        barycentrics: &Barycentrics,
    ) -> Self {
        Self {
            position: *position,
            in_direction: *in_direction,
            t,
            uv: *uv,
            normal: *normal,
            object_id,
            instance_id,
            primitive_id,
            material_id,
            barycentrics: *barycentrics,
        }
    }
}

pub struct TraceResult {
    location: FragCoord,
    intersection: Option<Intersection>,
}

pub struct Bounce {
    color: Color,
    out_dir: Direction,
}

pub trait Material {
    fn brdf(&self, surface: &Intersection) -> Option<Bounce>;
    fn pdf(&self, surface: &Intersection) -> f32;
}

pub trait Texture {
    fn sample(&self, uv: &TextureCoordinate, position: &Position) -> Color;
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
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
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
    albedo: Box<dyn Texture>,
}

impl DiffuseMaterial {
    fn new(albedo: Box<dyn Texture>) -> Self {
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

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
}

pub struct Sphere {
    radius: f32,
    position: Position,
    material_id: u32,
}

impl Sphere {
    fn new(radius: f32, position: &Position, material_id: u32) -> Self {
        Self {
            radius,
            position: *position,
            material_id,
        }
    }
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let oc = ray.origin - self.position;
        let a = dot(ray.dir, ray.dir);
        let half_b = dot(oc, ray.dir);
        let r2 = self.radius * self.radius;
        let c = dot(oc, oc) - r2;

        let discr = half_b * half_b - a * c;

        if discr < 0. {
            return None;
        }

        let sqrtd = discr.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let n = (p - self.position) / self.radius;
        let n = if dot(n, ray.dir) < 0. { n } else { -n };

        return Some(Intersection::new(
            &p,
            &ray.dir,
            root,
            &n,
            &TextureCoordinate::new(0., 0.),
            0,
            0,
            0,
            self.material_id,
            &Barycentrics::new(0., 0.),
        ));
    }
}

pub trait RayGenerationShader<Context> {
    fn generate(
        &self,
        ray_tracer: &dyn RayTracer<Context>,
        context: &mut Context,
        scene: &Scene<Context>,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    );
}

pub trait ClosestHitShader<Context> {
    fn hit(&self, ctx: &mut Context, intersection: &Intersection);
}

pub struct Instance {
    object_id: u32,
    hit_shader_id: u32,
    transform: Matrix4x3<f32>,
}

pub struct Scene<Context> {
    hittables: Vec<Box<dyn Hittable>>,
    instances: Vec<Instance>,
    closest_hit_shaders: Vec<Box<dyn ClosestHitShader<Context>>>,
}

impl<Context> Scene<Context> {
    fn new() -> Self {
        Scene {
            hittables: Vec::new(),
            instances: Vec::new(),
            closest_hit_shaders: Vec::new(),
        }
    }

    fn add_hittable(&mut self, t: Box<dyn Hittable>) -> usize {
        self.hittables.push(t);
        self.hittables.len() - 1
    }
}

impl<Context> Hittable for Scene<Context> {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut t = f32::MAX;
        let mut intersection = None;

        for hittable in self.hittables.iter() {
            if let Some(hit) = hittable.intersect(ray, t_min, t_max) {
                if hit.t < t {
                    t = hit.t;
                    intersection = Some(hit);
                }
            }
        }

        return intersection;
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

pub trait RayTracer<Context> {
    fn trace(&self, context: &mut Context, width: u32, height: u32, scene: &Scene<Context>);
    fn intersect(
        &self,
        context: &mut Context,
        scene: &Scene<Context>,
        ray: &Ray,
    ) -> Option<Intersection>;
}

pub struct CPUTracer<Context> {
    ray_generation_shader: Box<dyn RayGenerationShader<Context>>,
}

impl<Context> CPUTracer<Context> {
    fn new(ray_generation_shader: Box<dyn RayGenerationShader<Context>>) -> Self {
        Self {
            ray_generation_shader,
        }
    }
}

impl<Context> RayTracer<Context> for CPUTracer<Context> {
    fn trace(&self, context: &mut Context, width: u32, height: u32, scene: &Scene<Context>) {
        for y in (0..height - 1).rev() {
            for x in 0..width {
                self.ray_generation_shader
                    .generate(self, context, scene, width, height, x, y);
            }

            let progress = 1. - y as f32 / height as f32;
            println!("Progress: {}", progress * 100.);
        }
    }

    fn intersect(
        &self,
        context: &mut Context,
        scene: &Scene<Context>,
        ray: &Ray,
    ) -> Option<Intersection> {
        scene.intersect(ray, 0.01, 1000.)
    }
}

struct RayGenerator<MyContext> {
    camera: CameraSettings,
    ctx: std::marker::PhantomData<MyContext>,
}

fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn rand_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand_float()
}

fn rand_vec() -> Vector3<f32> {
    Vector3::new(rand_float(), rand_float(), rand_float())
}

fn rand_vec_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    )
}

fn rand_sphere() -> Vector3<f32> {
    loop {
        let p = rand_vec_range(-1., 1.);
        if length(p) >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

impl RayGenerationShader<MyContext> for RayGenerator<MyContext> {
    fn generate(
        &self,
        ray_tracer: &dyn RayTracer<MyContext>,
        context: &mut MyContext,
        scene: &Scene<MyContext>,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) {
        let mut color = Color::new(0., 0., 0.);
        for _ in 0..context.spp {
            let mut coefficient = Vector3::new(1., 1., 1.);
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

        let r = (sqrt(color.x) * 255.) as u8;
        let g = (sqrt(color.y) * 255.) as u8;
        let b = (sqrt(color.z) * 255.) as u8;

        context.output_image.put_pixel(x, y, Rgb([r, g, b]));
    }
}

struct MyContext {
    output_image: RgbImage,
    accumulation_buffer: Vec<Color>,
    spp: u32,
    max_depth: u32,
    materials: Vec<Box<dyn Material>>,
}

fn main() {
    let width = 1280;
    let height = 720;
    let mut scene = Scene::<MyContext>::new();
    let camera = CameraSettings::new(
        &Position::new(13., 2., 3.),
        &Direction::new(0., 0., 0.),
        16. / 9.,
        65.,
    );
    let mut ctx = MyContext {
        output_image: image::ImageBuffer::new(width, height),
        accumulation_buffer: vec![Color::new(0., 0., 0.); (width * height) as usize],
        spp: 4,
        max_depth: 4,
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

    // Floor
    scene.add_hittable(Box::new(Sphere::new(
        1000.,
        &Position::new(0., -1000., 0.),
        0,
    )));

    for _ in 0..20 {
        scene.add_hittable(Box::new(Sphere::new(
            rand_range(0.5, 1.25),
            &Position::new(
                rand_range(-10., 10.),
                rand_range(1., 5.),
                rand_range(-10., 10.),
            ),
            1,
        )));
    }
    scene.add_hittable(Box::new(Sphere::new(1., &Position::new(0., 1., 0.), 1)));
    let tracer = CPUTracer::new(Box::new(RayGenerator {
        camera: camera,
        ctx: std::marker::PhantomData::<MyContext>::default(),
    }));

    tracer.trace(&mut ctx, width, height, &scene);
    ctx.output_image.save("output.png");
}
