use super::intersection::*;
use super::ray::Ray;
use super::resources::Resources;
use super::types::*;

#[derive(Default)]
pub struct Bounce {
    pub ray: Ray,
    pub color: Color,
    pub pdf: f32,
}

impl Bounce {
    pub fn new(ray: &Ray, color: &Color, pdf: f32) -> Self {
        Self {
            ray: *ray,
            color: *color,
            pdf,
        }
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
    fn scatter(&self, resources: &Resources, _hit_record: &HitRecord) -> Bounce {
        Bounce::default()
    }

    fn emit(&self, resources: &Resources, _hit_record: &HitRecord) -> Option<Color> {
        None
    }
}
