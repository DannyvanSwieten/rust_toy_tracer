use crate::types::*;
use crate::vec::*;

pub trait Saturate {
    type Item;
    fn saturate(&self) -> Self::Item;
}

impl<const N: usize> Saturate for Vector<N> {
    type Item = Vector<N>;

    fn saturate(&self) -> Self::Item {
        let mut data: [f32; N] = [0.0; N];
        for i in 0..N {
            data[i] = self.data[i].clamp(0., 1.);
        }

        Vector::<N>::from_values(data)
    }
}

impl Saturate for f32 {
    type Item = f32;

    fn saturate(&self) -> Self::Item {
        self.clamp(0., 1.)
    }
}

pub fn mix(a: f32, b: f32, v: f32) -> f32 {
    a * (1f32 - v) + b * v
}

pub fn mix_vec3(a: &Vec3, b: &Vec3, v: f32) -> Vec3 {
    a * (1f32 - v) + b * v
}

pub fn pow2(x: f32) -> f32 {
    x * x
}

pub fn same_hemisphere(wo: &Direction, wi: &Direction, normal: &Direction) -> bool {
    return dot(wo, normal) * dot(wi, normal) > 0.0;
}
