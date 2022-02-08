use std::f32::consts::PI;
use std::f32::consts::SQRT_2;

use super::types::*;
use super::vec::*;
use rand::Rng;

pub fn float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn int_range(min: u32, max: u32) -> u32 {
    float_range(min as f32, max as f32) as u32
}

pub fn float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * float()
}

pub fn vec() -> Vec3 {
    Vec3::from_values(&[float(), float(), float()])
}

pub fn vec_range(min: f32, max: f32) -> Vec3 {
    Vec3::from_values(&[
        float_range(min, max),
        float_range(min, max),
        float_range(min, max),
    ])
}

pub fn sphere() -> Direction {
    loop {
        let p = vec_range(-1., 1.);
        if length(&p) >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}

pub fn cosine() -> Direction {
    let r1 = float();
    let r2 = float();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;
    let x = phi.cos() * SQRT_2;
    let y = phi.sin() * SQRT_2;
    Direction::from_values(&[x, y, z])
}

pub fn disk() -> Direction {
    loop {
        let p = Direction::from_values(&[float_range(-1.0, 1.0), float_range(-1.0, 1.0), 0.0]);
        if length(&p) >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
