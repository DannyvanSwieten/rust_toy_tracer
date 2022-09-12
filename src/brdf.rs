use crate::{
    math_utils::mix_vec3,
    onb::OrthoNormalBasis,
    rand::float,
    types::{Color, Direction, Vec3},
};

use super::vec::*;
use std::{
    f32::consts::{PI, TAU},
    mem::swap,
};

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

pub fn fresnel_schlick(cos_theta: f32, f0: &Color) -> Color {
    *f0 + (Color::ones() - f0) * (1.0 - cos_theta).powf(5.)
}

pub fn g1_ggx_schlick(n_dot_v: f32, roughness: f32) -> f32 {
    let r = roughness;
    let k = (r * r) / 2.0;
    let denom = n_dot_v * (1.0 - k) + k;
    n_dot_v / denom
}

pub fn g_smith(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
    let g1_l = g1_ggx_schlick(n_dot_v, roughness);
    let g1_v = g1_ggx_schlick(n_dot_l, roughness);
    g1_v * g1_l
}

pub fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
    let alpha = roughness * roughness;
    let alpha = alpha * alpha;
    let n_dot_h2 = n_dot_h * n_dot_h;
    let b = n_dot_h2 * (alpha - 1.0) + 1.0;
    alpha / (PI * b * b)
}

pub fn sample_microfacet_transmission_brdf(
    v: &Direction,
    normal: &Direction,
    albedo: &Color,
    metal: f32,
    roughness: f32,
    fresnel: f32,
    ior: f32,
) -> (Direction, Color) {
    let x = float();
    let y = float();

    let mut forward_normal = *normal;
    let mut eta = 1.0 / ior;
    if dot(v, normal) < 0.0 {
        eta = ior;
        forward_normal = -normal;
    }

    let a = roughness * roughness;
    let theta = (1.0 - y) / (1.0 + (a * a - 1.0) * y);
    let theta = theta.sqrt().acos();
    let phi = TAU * x;
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let local_h = Direction::from_values([sin_theta * cos_phi, sin_theta * sin_phi, cos_theta]);
    let h = normalize(&OrthoNormalBasis::from_w(&forward_normal).local(&local_h));
    let l = refract_glsl(&-v, &h, eta);

    let n_dot_v = saturate(dot(&forward_normal, v));
    let n_dot_l = saturate(dot(&-forward_normal, &l));
    let n_dot_h = saturate(dot(&forward_normal, &h));
    let v_dot_h = saturate(dot(&v, &h));

    let f0 = Color::splat(0.16 * (fresnel * fresnel));
    let f0 = mix_vec3(&f0, albedo, metal);

    let f = fresnel_schlick(v_dot_h, &f0);
    let g = g_smith(n_dot_v, n_dot_l, roughness);
    let color = Color::ones() - f;
    let color = *albedo * color * g * v_dot_h / (n_dot_h * n_dot_v).max(0.001);
    (l, color)
}

pub fn sample_diffuse_brdf(
    v: &Direction,
    albedo: &Color,
    metal: f32,
    fresnel: f32,
) -> (Direction, Color) {
    let x = float();
    let y = float();
    let theta = y.sqrt().asin();
    let phi = TAU * x;
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let l = Direction::from_values([sin_theta * cos_phi, sin_theta * sin_phi, cos_theta]);
    let h = *v + l;
    let v_dot_h = saturate(dot(v, &h));
    let f0 = Color::splat(0.16 * fresnel * fresnel);
    let f0 = mix_vec3(&f0, albedo, metal);
    let f = fresnel_schlick(v_dot_h, &f0);
    let not_specular = Color::ones() - f;
    let diffuse = not_specular * albedo * (1.0 - metal);
    (l, diffuse)
}

pub fn sample_micro_facet_isotropic_specular_brdf(
    v: &Direction,
    normal: &Direction,
    albedo: &Color,
    metal: f32,
    roughness: f32,
    fresnel: f32,
) -> (Direction, Color) {
    let x = float();
    let y = float();

    let a = roughness * roughness;
    let theta = (1.0 - y) / (1.0 + (a * a - 1.0) * y);
    let theta = theta.sqrt().acos();
    let phi = TAU * x;
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let local_h = Direction::from_values([sin_theta * cos_phi, sin_theta * sin_phi, cos_theta]);
    let h = OrthoNormalBasis::from_w(normal).local(&local_h);
    let l = reflect(&-v, &h);

    let n_dot_v = saturate(dot(normal, v));
    let n_dot_l = saturate(dot(normal, &l));
    let n_dot_h = saturate(dot(normal, &h));
    let v_dot_h = saturate(dot(&v, &h));

    let f0 = Color::splat(0.16 * fresnel * fresnel);
    let f0 = mix_vec3(&f0, albedo, metal);

    let f = fresnel_schlick(v_dot_h, &f0);
    let g = g_smith(n_dot_v, n_dot_l, roughness);
    let specular = f * g * v_dot_h / (n_dot_h * n_dot_v).max(0.001);
    (l, specular)
}

pub fn evaluate_microfacet_isotropic_brdf(
    l: &Direction,
    v: &Direction,
    normal: &Direction,
    albedo: &Color,
    metal: f32,
    roughness: f32,
    transmission: f32,
) -> Color {
    let h = *l + *v;
    let h = normalize(&h);
    let n_dot_v = saturate(dot(&normal, v));
    let n_dot_l = saturate(dot(&normal, l));
    let n_dot_h = saturate(dot(&normal, &h));
    let v_dot_h = saturate(dot(&normal, v));

    let f0 = Color::splat(0.04);
    let f0 = mix_vec3(&f0, albedo, metal);

    let f = fresnel_schlick(v_dot_h, &f0);
    let d = distribution_ggx(n_dot_h, roughness);
    let g = g_smith(n_dot_v, n_dot_l, roughness);
    let specular = (d * g * f) / (4.0 * n_dot_v * n_dot_l).max(0.001);

    let mut not_specular = Color::ones() - f;
    not_specular *= (1.0 - metal) * (1.0 - transmission);
    let diffuse = not_specular * albedo / PI;
    diffuse + specular
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
