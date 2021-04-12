use super::types::*;
use super::vec::*;
use rand::Rng;

pub fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand_float()
}

pub fn rand_vec() -> Vec3 {
    Vec3::from_values(&[rand_float(), rand_float(), rand_float()])
}

pub fn rand_vec_range(min: f32, max: f32) -> Vec3 {
    Vec3::from_values(&[
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    ])
}

pub fn rand_sphere() -> Position {
    loop {
        let p = rand_vec_range(-1., 1.);
        if length(&p) >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
