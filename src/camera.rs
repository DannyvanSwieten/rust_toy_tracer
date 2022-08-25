use crate::ray::Ray;

pub trait Camera {
    fn ray(x: usize, y: usize) -> Ray;
}
