use glm::builtin::*;
use glm::Vector3;
use rand::Rng;

pub fn rand_float() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range(min: f32, max: f32) -> f32 {
    min + (max - min) * rand_float()
}

pub fn rand_vec() -> Vector3<f32> {
    Vector3::new(rand_float(), rand_float(), rand_float())
}

pub fn rand_vec_range(min: f32, max: f32) -> Vector3<f32> {
    Vector3::new(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    )
}

pub fn rand_sphere() -> Vector3<f32> {
    loop {
        let p = rand_vec_range(-1., 1.);
        if length(p) >= 1.0 {
            continue;
        } else {
            return p;
        }
    }
}
