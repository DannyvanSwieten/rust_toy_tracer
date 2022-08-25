use std::f32::consts::PI;
use std::f32::consts::TAU;

use super::rand;
use super::types::*;
use crate::disney_brdf_pdf::pdf_disney;
use crate::disney_brdf_pdf::pdf_lambert;
use crate::onb::OrthoNormalBasis;
use crate::rand::cosine;
use crate::vec::dot;
use crate::{
    disney_brdf_pdf::pdf_disney_micro_facet_anisotropic,
    math_utils::same_hemisphere,
    vec::{normalize, reflect, XAccessor, YAccessor, ZAccessor},
};

pub fn sample_disney_diffuse(wo: &Direction, normal: &Direction) -> Direction {
    let local = cosine();
    let mut wi = OrthoNormalBasis::from_w(normal).local(&local);
    if dot(&wo, normal) < 0.0 {
        wi.data[2] *= -1.0
    }

    wi
}

pub fn sample_disney_sub_surface(wo: &Direction, normal: &Direction) -> Direction {
    let local = cosine();
    let mut wi = OrthoNormalBasis::from_w(&local).local(normal);
    if dot(wo, normal) < 0.0 {
        wi.data[2] *= -1.0;
    }

    wi
}

pub fn sample_disney_micro_facet_anisotropic(
    wo: &Direction,
    normal: &Direction,
    x: &Direction,
    y: &Direction,
    roughness: f32,
    anisotropy: f32,
) -> Direction {
    let xi = Vec2::from_values([rand::float(), rand::float()]);
    let roughness_2 = roughness * roughness;

    let aspect = (1. - anisotropy * 0.9).sqrt();
    let alphax = (roughness_2 / aspect).max(0.001);
    let alphay = (roughness_2 * aspect).max(0.001);

    let mut phi = (alphay / alphax * (TAU * xi.y() + 0.5 * PI).tan()).atan();

    if xi.y() > 0.5 {
        phi += PI;
    }

    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let alphax2 = alphax * alphax;
    let alphay2 = alphay * alphay;
    let alpha2 = 1. / (cos_phi * cos_phi / alphax2 + sin_phi * sin_phi / alphay2).max(0.001);
    let tan_theta2 = alpha2 * xi.x() / (1. - xi.x());
    let cos_theta = 1. / (1. + tan_theta2).sqrt();
    let sin_theta = (1. - cos_theta * cos_theta).max(0.0).sqrt();

    let wh_local = Direction::from_values([sin_theta * cos_phi, sin_theta * sin_phi, cos_theta]);
    let mut wh = wh_local.x() * x + wh_local.y() * y + wh_local.z() * normal;
    if !same_hemisphere(wo, &wh, normal) {
        wh *= -1.;
    }

    let wi = normalize(&reflect(&-wo, &wh));
    wi
}

pub fn sample_disney_bsdf(
    wo: &Direction,
    normal: &Direction,
    x: &Direction,
    y: &Direction,
    roughness: f32,
    anisotropy: f32,
) -> (Direction, f32) {
    let r = rand::float();
    let wi = if r < 0.5 {
        sample_disney_diffuse(wo, normal)
    } else {
        sample_disney_micro_facet_anisotropic(wo, normal, x, y, roughness, anisotropy)
    };

    (wi, pdf_disney(&wi, wo, normal, x, y, roughness, anisotropy))
}
