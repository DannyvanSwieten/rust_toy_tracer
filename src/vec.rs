use super::types::*;
use super::vec_mul;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<const SIZE: usize> {
    pub data: [f32; SIZE],
}

impl<const SIZE: usize> std::ops::Index<usize> for Vector<SIZE> {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<const SIZE: usize> std::ops::IndexMut<usize> for Vector<SIZE> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

impl<const SIZE: usize> Vector<SIZE> {
    pub fn new() -> Self {
        Self { data: [0.; SIZE] }
    }

    pub fn from_values(data: &[f32; SIZE]) -> Self {
        Self { data: *data }
    }
}

pub fn dot<const SIZE: usize>(lhs: &Vector<SIZE>, rhs: &Vector<SIZE>) -> f32 {
    let mut sum = 0.;
    for i in 0..SIZE {
        sum += lhs.data[i] * rhs.data[i]
    }
    sum
}

pub fn min<const SIZE: usize>(lhs: &Vector<SIZE>, rhs: &Vector<SIZE>) -> Vector<SIZE> {
    let mut result = Vector::<SIZE>::new();
    for i in 0..SIZE {
        result.data[i] = lhs.data[i].min(rhs.data[i])
    }

    result
}

pub fn max<const SIZE: usize>(lhs: &Vector<SIZE>, rhs: &Vector<SIZE>) -> Vector<SIZE> {
    let mut result = Vector::<SIZE>::new();
    for i in 0..SIZE {
        result.data[i] = lhs.data[i].max(rhs.data[i])
    }

    result
}

pub fn length<const SIZE: usize>(v: &Vector<SIZE>) -> f32 {
    dot(v, v).sqrt()
}

pub fn normalize<const SIZE: usize>(v: &Vector<SIZE>) -> Vector<SIZE> {
    let inv_d = 1. / length(v);
    (*v) * inv_d
}

pub fn cross(lhs: &Vec3, rhs: &Vec3) -> Vec3 {
    let x = lhs.y() * rhs.z() - rhs.y() * lhs.z();
    let y = lhs.z() * rhs.x() - rhs.z() * lhs.x();
    let z = lhs.x() * rhs.y() - rhs.x() * lhs.y();

    Vec3::from_values(&[x, y, z])
}

pub fn reflect(i: &Vec3, n: &Vec3) -> Vec3 {
    let d = dot(i, n) * 2.;
    (*i) - (*n) * d
}

pub trait XAccessor {
    fn x(&self) -> f32;
}

pub trait YAccessor {
    fn y(&self) -> f32;
}

pub trait ZAccessor {
    fn z(&self) -> f32;
}

pub trait WAccessor {
    fn w(&self) -> f32;
}

impl<const SIZE: usize> XAccessor for Vector<SIZE> {
    fn x(&self) -> f32 {
        self.data[0]
    }
}

impl<const SIZE: usize> YAccessor for Vector<SIZE> {
    fn y(&self) -> f32 {
        self.data[1]
    }
}

impl<const SIZE: usize> ZAccessor for Vector<SIZE> {
    fn z(&self) -> f32 {
        self.data[2]
    }
}

impl<const SIZE: usize> WAccessor for Vector<SIZE> {
    fn w(&self) -> f32 {
        self.data[3]
    }
}

impl From<Vec3> for Vec4 {
    fn from(v: Vec3) -> Self {
        Vec4::from_values(&[v.x(), v.y(), v.z(), 1.])
    }
}
