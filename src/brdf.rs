use crate::onb::OrthoNormalBasis;
use crate::types::{Color, Direction, Vec3};

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

pub fn fresnel_schlick_refraction(cos: f32, ior: f32) -> f32 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
}

pub fn ggx_normal_distribution(n_dot_h: f32, roughness: f32) -> f32 {
    let a2 = roughness * roughness;
    let d = (n_dot_h * a2 - n_dot_h) * n_dot_h + 1.0;
    let result = a2 / (d * d * PI).max(0.000001);
    result
}

pub fn schlick_masking_term(n_dot_l: f32, n_dot_v: f32, roughness: f32) -> f32 {
    let k = roughness * roughness / 2.0;
    let g_v = n_dot_v / (n_dot_v * (1.0 - k) + k);
    let g_l = n_dot_l / (n_dot_l * (1.0 - k) + k);
    g_v * g_l
}

pub fn schlick_fresnel(f0: &Color, l_dot_h: f32) -> Color {
    *f0 + (Color::splat(1.0) - f0) * (1.0 - l_dot_h).powf(5.0)
}

pub fn ggx_micro_facet_normal(roughness: f32, hit_normal: &Direction) -> Direction {
    let r1 = rand::float();
    let r2 = rand::float();
    let a2 = roughness * roughness;
    let cos_theta = (1.0 - r1) / ((a2 - 1.0) * r1 + 1.0);
    let cos_theta = cos_theta.max(0.0).sqrt();
    let sin_theta = (1.0 - cos_theta - cos_theta).max(0.0).sqrt();
    let phi = r2 * PI * 2.0;

    let onb = OrthoNormalBasis::from_w(&hit_normal);
    onb.u() * (sin_theta * phi.cos()) + onb.v() * (sin_theta * phi.sin()) + onb.w() * cos_theta
}
