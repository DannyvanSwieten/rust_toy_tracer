use super::acceleration_structure::BottomLevelAccelerationStructure;
use super::bounding_box::*;
use super::intersection::*;
use super::ray::*;
use super::types::*;
use super::vec::*;
use std::time::Instant;

pub trait Hittable {
    fn uid(&self) -> usize;
    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        cull: bool,
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
}

impl Sphere {
    pub fn new(radius: f32, position: &Position) -> Self {
        Self {
            radius,
            position: *position,
        }
    }
}

impl Hittable for Sphere {
    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        cull: bool,
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

        if discr < 0.000001 {
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

        return Some(Intersection::new(ray, root, 0, &Barycentrics::new()));
    }

    fn normal(&self, object_to_world: &Transform, intersection: &Intersection) -> Normal {
        let n =
            intersection.ray.at(intersection.t) - (*object_to_world * Vec4::from(self.position));
        normalize(&n)
    }

    fn uv(&self, _: &Transform, _intersection: &Intersection) -> TextureCoordinate {
        TextureCoordinate::new()
    }

    fn bounding_box(&self) -> std::option::Option<BoundingBox> {
        let r = Position::from_values([self.radius, self.radius, self.radius]);
        Some(BoundingBox::new(self.position - r, self.position + r))
    }

    fn uid(&self) -> usize {
        1
    }
}

pub struct TriangleMesh {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    tex_coords: Vec<TextureCoordinate>,
    indices: Vec<u32>,
    acceleration_structure: BottomLevelAccelerationStructure,
}

impl Hittable for TriangleMesh {
    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        cull: bool,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        let mut intersection = None;

        const USE_ACCELERATION_STRUCTURE: bool = true;
        let start = Instant::now();
        if !USE_ACCELERATION_STRUCTURE {
            let mut closest = t_max;
            for i in 0..self.indices.len() / 3 {
                let index = i * 3;
                let i0 = self.indices[index] as usize;
                let i1 = self.indices[index + 1] as usize;
                let i2 = self.indices[index + 2] as usize;

                let v0 = *object_to_world * &Vec4::from(self.positions[i0]);
                let v1 = *object_to_world * &Vec4::from(self.positions[i1]);
                let v2 = *object_to_world * &Vec4::from(self.positions[i2]);

                if let Some((t, u, v)) =
                    self.ray_triangle_intersect(ray, cull, t_min, t_max, &v0, &v1, &v2)
                {
                    if t < closest {
                        intersection = Some(Intersection::new(
                            ray,
                            t,
                            index as u32,
                            &Barycentrics::from_values([u, v]),
                        ));

                        closest = t;
                    }
                }
            }
        } else {
            let result = self
                .acceleration_structure
                .hit_test(object_to_world, ray, t_min, t_max);

            for index in result {
                let i = index as usize * 3;
                let i0 = self.indices[i as usize] as usize;
                let i1 = self.indices[i + 1] as usize;
                let i2 = self.indices[i + 2] as usize;

                let v0 = *object_to_world * &Vec4::from(self.positions[i0]);
                let v1 = *object_to_world * &Vec4::from(self.positions[i1]);
                let v2 = *object_to_world * &Vec4::from(self.positions[i2]);

                let mut closest = t_max;
                if let Some((t, u, v)) =
                    self.ray_triangle_intersect(ray, cull, t_min, t_max, &v0, &v1, &v2)
                {
                    if t < closest {
                        intersection = Some(Intersection::new(
                            ray,
                            t,
                            i as u32,
                            &Barycentrics::from_values([u, v]),
                        ));

                        closest = t;
                    }
                }
            }
        }
        let duration = start.elapsed();
        //println!("Time elapsed in object trace is: {:?}", duration);
        intersection
    }

    fn normal(&self, object_to_world: &Transform, intersection: &Intersection) -> Normal {
        let i = intersection.primitive_id as usize;
        let i0 = self.indices[i] as usize;
        let i1 = self.indices[1 + i] as usize;
        let i2 = self.indices[2 + i] as usize;

        let tr = object_to_world.transposed();

        let n1 = tr
            * self.normals[i0]
            * (1. - intersection.barycentrics.x() - intersection.barycentrics.y());
        let n2 = tr * self.normals[i1] * intersection.barycentrics.x();
        let n3 = tr * self.normals[i2] * intersection.barycentrics.y();
        let n = n1 + n2 + n3;
        normalize(&Vec3::from(n))
    }
    fn uv(&self, _: &Transform, intersection: &Intersection) -> TextureCoordinate {
        let i = intersection.primitive_id as usize;
        let i0 = self.indices[i] as usize;
        let i1 = self.indices[1 + i] as usize;
        let i2 = self.indices[2 + i] as usize;

        let t1 = self.tex_coords[i0]
            * (1. - intersection.barycentrics.x() - intersection.barycentrics.y());
        let t2 = self.tex_coords[i1] * intersection.barycentrics.x();
        let t3 = self.tex_coords[i2] * intersection.barycentrics.y();
        t1 + t2 + t3
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(self.acceleration_structure.bounding_box())
    }

    fn uid(&self) -> usize {
        2
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
            for index in (0..indices.len()).step_by(3) {
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

        for n in normals.iter_mut() {
            let new_n = normalize(n);
            n[0] = new_n[0];
            n[1] = new_n[1];
            n[2] = new_n[2];
        }

        if tex_coords.len() == 0 {
            tex_coords.resize(positions.len(), TextureCoordinate::new())
        }

        let acceleration_structure =
            BottomLevelAccelerationStructure::new(&positions, Some(&indices));

        Self {
            positions,
            normals,
            tex_coords,
            indices,
            acceleration_structure,
        }
    }

    fn ray_triangle_intersect(
        &self,
        ray: &Ray,
        cull: bool,
        _t_min: f32,
        _t_max: f32,
        v0: &Position,
        v1: &Position,
        v2: &Position,
    ) -> Option<(f32, f32, f32)> {
        let v0v1 = *v1 - v0;
        let v0v2 = *v2 - v0;
        let pvec = cross(ray.direction(), &v0v2);
        let det = dot(&v0v1, &pvec);

        if cull {
            if det < 0.000001 {
                return None;
            }
        } else {
            if det.abs() < 0.000001 {
                return None;
            }
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

pub struct XYRect {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl Hittable for XYRect {
    fn intersect(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        cull: bool,
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection> {
        todo!()
    }

    fn normal(&self, object_to_world: &Transform, intersection: &Intersection) -> Normal {
        *object_to_world * &Vec4::from(Normal::from_values([0.0, 0.0, -1.0]))
    }

    fn uv(&self, object_to_world: &Transform, intersection: &Intersection) -> TextureCoordinate {
        Vec2::from_values([0.0, 0.0])
    }

    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(BoundingBox::new(
            Vec3::from_values([self.x0, self.y0, -0.0001]),
            Vec3::from_values([self.x1, self.y1, 0.0001]),
        ))
    }

    fn uid(&self) -> usize {
        3
    }
}
pub struct XZRect {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}
pub struct YZRect {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}
