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

pub fn sphere() -> Position {
    loop {
        let p = vec_range(-1., 1.);
        if length(&p) >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
