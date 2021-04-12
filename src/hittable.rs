use super::bounding_box::*;
use super::intersection::*;
use super::ray::*;
use super::types::*;
use super::vec::*;

pub trait Hittable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
    fn bounding_box(&self) -> Option<BoundingBox>;
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
        let a = dot(&ray.dir, &ray.dir);
        let half_b = dot(&oc, &ray.dir);
        let r2 = self.radius * self.radius;
        let c = dot(&oc, &oc) - r2;

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
        let n = if dot(&n, &ray.dir) < 0. { n } else { -n };

        return Some(Intersection::new(
            &p,
            &ray.dir,
            root,
            &n,
            &TextureCoordinate::from_values(&[0., 0.]),
            0,
            0,
            0,
            self.material_id,
            &Barycentrics::from_values(&[0., 0.]),
        ));
    }
    fn bounding_box(&self) -> std::option::Option<BoundingBox> {
        let r = Position::from_values(&[self.radius, self.radius, self.radius]);
        Some(BoundingBox::new(&(self.position - r), &(self.position + r)))
    }
}

pub struct TriangleMesh {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    tex_coords: Vec<TextureCoordinate>,
    indices: Vec<u32>,
}

impl Hittable for TriangleMesh {
    fn bounding_box(&self) -> Option<BoundingBox> {
        let mut min_p = Position::from_values(&[std::f32::MIN, std::f32::MIN, std::f32::MIN]);
        let mut max_p = Position::from_values(&[std::f32::MAX, std::f32::MAX, std::f32::MAX]);
        for p in self.positions.iter() {
            min_p = min(&min_p, p);
            max_p = max(&max_p, p);
        }

        Some(BoundingBox::new(&min_p, &max_p))
    }
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        for i in self.indices.iter().step_by(3) {
            let v0 = &self.positions[*i as usize];
            let v1 = &self.positions[(*i + 1) as usize];
            let v2 = &self.positions[(*i + 2) as usize];

            if let Some(hit) = self.ray_triangle_intersect(ray, t_min, t_max, v0, v1, v2) {
                return Some(hit);
            }
        }

        None
    }
}

impl TriangleMesh {
    fn ray_triangle_intersect(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        v0: &Position,
        v1: &Position,
        v2: &Position,
    ) -> Option<Intersection> {
        None
    }
}
