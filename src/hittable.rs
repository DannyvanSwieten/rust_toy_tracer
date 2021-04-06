use super::intersection::*;
use super::ray::*;
use super::types::*;
use glm::builtin::*;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
}

pub struct Sphere {
    radius: f32,
    position: Position,
    material_id: u32,
}

impl Sphere {
    pub fn new(radius: f32, position: &Position, material_id: u32) -> Self {
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
