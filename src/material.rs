use super::intersection::*;
use super::ray::Ray;
use super::resources::Resources;
use super::types::*;

#[derive(Default)]
pub struct Bounce {
    pub ray: Ray,
    pub pdf: f32,
}

impl Bounce {
    pub fn new(ray: &Ray, pdf: f32) -> Self {
        Self { ray: *ray, pdf }
    }
}

#[derive(Default)]
pub struct HitRecord {
    pub intersection: Intersection,
    pub normal: Direction,
    pub uv: TextureCoordinate,
    pub front_facing: bool,
    pub instance_id: u32,
    pub bounce: Bounce,
    pub direct_light: Color,
}

impl HitRecord {
    pub fn position(&self) -> Position {
        self.intersection.ray.at(self.intersection.t)
    }

    pub fn barycentrics(&self) -> &Barycentrics {
        &self.intersection.barycentrics
    }

    pub fn ray_direction(&self) -> &Direction {
        self.intersection.ray.direction()
    }
}

pub trait Material {
    fn uid(&self) -> usize;

    fn scatter(&self, _: &Resources, _hit_record: &HitRecord) -> Bounce {
        Bounce::default()
    }

    fn evaluate(&self, _: &Resources, _hit_record: &HitRecord) -> Color;

    fn emit(&self, _: &Resources, _hit_record: &HitRecord) -> Color {
        Color::new()
    }
}
