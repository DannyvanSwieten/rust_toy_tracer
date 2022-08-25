use crate::onb::OrthoNormalBasis;
use crate::types::{Color, Direction, Vec2, Vec3};

use super::rand;
use super::vec::*;
use std::f32::consts::PI;
use std::mem::swap;

pub fn saturate(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

pub fn fresnel(i: &Vec3, n: &Vec3, ior: f32) -> f32 {
    let cosi = dot(i, n).clamp(-1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = ior;
    if cosi > 0.0 {
        swap(&mut etai, &mut etat);
    }

    let sint = etai / etat * (1.0 - cosi * cosi).max(0.0).sqrt();
    if sint >= 1.0 {
        1.0
    } else {
        let cost = (1.0 - sint * sint).max(0.0).sqrt();
        let cosi = cosi.abs();
        let etat_cosi = etat * cosi;
        let etai_cost = etai * cost;
        let etai_cosi = etai * cosi;
        let etat_cost = etat * cost;
        let rs = (etat_cosi - etai_cost) / (etat_cosi + etai_cost);
        let rp = (etai_cosi - etat_cost) / (etai_cosi + etat_cost);
        (rs * rs + rp * rp) / 2.0
    }
}

pub fn fresnel_schlick_reflectance(cos: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

#[test]
fn test_reflectance() {
    let cosine = 0.0;
    let ref_idx = 1.5;
    let expected = 1.0;
    let actual = fresnel_schlick_reflectance(cosine, ref_idx);
    assert_eq!(actual, expected);
}

#[test]
fn test_refract() {
    let uv = Direction::from_values([1.0, 1.0, 0.0]);
    let n = Direction::from_values([-1.0, 0.0, 0.0]);
    let etai_over_etat = 1.0;
    let expected = Direction::from_values([0.0, 1.0, 0.0]);
    let actual = refract(&uv, &n, etai_over_etat);
    assert_eq!(actual, expected);
}
