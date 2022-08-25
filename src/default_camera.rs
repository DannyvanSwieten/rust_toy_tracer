use super::rand;
use super::types::*;
use crate::vec::XAccessor;
use crate::vec::YAccessor;
use crate::{
    degrees_to_radians,
    ray::Ray,
    vec::{cross, normalize},
};
pub struct DefaultCamera {
    origin: Position,
    left_corner: Position,
    horizontal: Direction,
    vertical: Direction,
    u: Direction,
    v: Direction,
    w: Direction,
    lens_radius: f32,
}

impl DefaultCamera {
    pub fn new(
        origin: &Position,
        look_at: &Direction,
        aspect_ratio: f32,
        fov: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = degrees_to_radians(fov);
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(&((*origin) - look_at));
        let up = Direction::from_values([0., 1., 0.]);
        let u = cross(&up, &w);
        let v = cross(&w, &u);

        let horizontal = focus_distance * u * viewport_width;
        let vertical = focus_distance * v * viewport_height;
        let left_corner = *origin - horizontal / 2. - vertical / 2. - focus_distance * w;
        Self {
            origin: *origin,
            left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    pub fn ray(&self, s: f32, t: f32) -> Ray {
        let rd = rand::disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            &(offset + self.origin),
            &normalize(
                &(self.left_corner + self.horizontal * s + self.vertical * t
                    - self.origin
                    - offset),
            ),
        )
    }
}
