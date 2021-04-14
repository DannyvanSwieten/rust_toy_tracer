use super::bounding_box::*;
use super::intersection::*;
use super::ray::*;
use super::types::*;
use super::vec::*;

pub trait Hittable {
    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection>;

    fn normal(&self, object_to_world: &Transform, intersection: &Intersection) -> Normal;
    fn uv(&self, object_to_world: &Transform, intersection: &Intersection) -> TextureCoordinate;
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
    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        let r = object_to_world.colums[0][0] * self.radius;
        let oc = ray.origin - (*object_to_world * Vec4::from(self.position));
        let a = dot(&ray.dir, &ray.dir);
        let half_b = dot(&oc, &ray.dir);
        let r2 = r * r;
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

        return Some(Intersection::new(
            &ray.at(root),
            &ray.dir,
            root,
            0,
            &Barycentrics::from_values(&[0., 0.]),
        ));
    }

    fn normal(&self, object_to_world: &Transform, intersection: &Intersection) -> Normal {
        let r = object_to_world.colums[0][0] * self.radius;
        let n = (intersection.position - (*object_to_world * Vec4::from(self.position))) / r;
        let n = if dot(&n, &intersection.in_direction) < 0. {
            n
        } else {
            -n
        };
        normalize(&n)
    }

    fn uv(&self, _: &Transform, intersection: &Intersection) -> TextureCoordinate {
        TextureCoordinate::new()
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
        let mut min_p = Position::from_values(&[std::f32::MAX, std::f32::MAX, std::f32::MAX]);
        let mut max_p = Position::from_values(&[std::f32::MIN, std::f32::MIN, std::f32::MIN]);
        for p in self.positions.iter() {
            min_p = min(&min_p, p);
            max_p = max(&max_p, p);
        }

        Some(BoundingBox::new(&min_p, &max_p))
    }

    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        for i in 0..self.indices.len() / 3 {
            let index = i * 3;
            let i0 = self.indices[index] as usize;
            let i1 = self.indices[index + 1] as usize;
            let i2 = self.indices[index + 2] as usize;

            let v0 = *object_to_world * &Vec4::from(self.positions[i0]);
            let v1 =
                *object_to_world * &Vec4::from(self.positions[i1]);
            let v2 =
                *object_to_world * &Vec4::from(self.positions[i2]);

            if let Some((t, u, v)) = self.ray_triangle_intersect(ray, t_min, t_max, &v0, &v1, &v2) {
                return Some(Intersection::new(
                    &ray.at(t),
                    ray.direction(),
                    t,
                    index as u32,
                    &Barycentrics::from_values(&[u, v]),
                ));
            }
        }

        None
    }
    fn normal(&self, object_to_world: &Transform, intersection: &Intersection) -> Normal {
        let i = intersection.primitive_id as usize;
        let i0 = self.indices[i] as usize;
        let i1 = self.indices[1 + i] as usize;
        let i2 = self.indices[2 + i] as usize;
        let n1 =
            *object_to_world * &Vec4::from(self.normals[i0]) * intersection.barycentrics.x();
        let n2 = *object_to_world * &Vec4::from(self.normals[i1]) * intersection.barycentrics.y();
        let n3 = *object_to_world * &Vec4::from(self.normals[i2]) *  (1. - intersection.barycentrics.x() - intersection.barycentrics.y());
        let n = n1 + n2 + n3;
        normalize(&n)
    }

    fn uv(&self, _: &Transform, intersection: &Intersection) -> TextureCoordinate {
        // let i = intersection.primitive_id as usize;
        // let i0 = self.indices[i] as usize;
        // let i1 = self.indices[1 + i] as usize;
        // let i2 = self.indices[2 + i] as usize;

        // let t1 = self.tex_coords[i0]
        //     * (1. - intersection.barycentrics.x() - intersection.barycentrics.y());
        // let t2 = self.tex_coords[i1] * intersection.barycentrics.x();
        // let t3 = self.tex_coords[i2] * intersection.barycentrics.y();
        // t1 + t2 + t3

        TextureCoordinate::new()
    }
}

impl TriangleMesh {
    pub fn new(
        positions: Vec<Position>,
        mut normals: Vec<Normal>,
        mut tex_coords: Vec<TextureCoordinate>,
        indices: Vec<u32>,
    ) -> Self {
        if normals.len() == 0 {
            normals.resize(positions.len(), Normal::new());
            for i in 0..indices.len() / 3 {
                let index = i * 3;

                let i0 = indices[index] as usize;
                let i1 = indices[index + 1] as usize;
                let i2 = indices[index + 2] as usize;

                let v0 = &positions[i0];
                let v1 = &positions[i1];
                let v2 = &positions[i2];

                let v1v0 = *v1 - v0;
                let v2v0 = *v2 - v0;
                let n = normalize(&cross(&v1v0, &v2v0));
                normals[i0] = normals[i0] + n;
                normals[i1] = normals[i1] + n;
                normals[i2] = normals[i2] + n;
            }
        }

        if tex_coords.len() == 0 {
            tex_coords.resize(positions.len(), TextureCoordinate::new())
        }

        Self {
            positions,
            normals,
            tex_coords,
            indices,
        }
    }

    fn ray_triangle_intersect(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        v0: &Position,
        v1: &Position,
        v2: &Position,
    ) -> Option<(f32, f32, f32)> {
        let v0v1 = *v1 - v0;
        let v0v2 = *v2 - v0;
        let pvec = cross(ray.direction(), &v0v2);
        let det = dot(&v0v1, &pvec);

        if det < 0.00001 {
            return None;
        }

        let inv_det = 1. / det;
        let tvec = *ray.origin() - v0;
        let u = dot(&pvec, &tvec) * inv_det;
        if u < 0. || u > 1. {
            return None;
        }

        let qvec = cross(&tvec, &v0v1);
        let v = dot(ray.direction(), &qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = dot(&v0v2, &qvec) * inv_det;
        Some((t, u, v))
    }
}
