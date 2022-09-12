use std::f32::consts::PI;

use crate::{
    disney_brdf_evaluate::gtr1,
    math_utils::{mix, pow2, same_hemisphere},
    vec::{dot, normalize},
};

use super::types::*;

pub fn pdf_lambert(wi: &Direction, wo: &Direction, normal: &Direction) -> f32 {
    if same_hemisphere(wo, wi, normal) {
        dot(normal, wi).abs() / PI
    } else {
        0f32
    }
}

pub fn pdf_clear_coat(
    wi: &Direction,
    wo: &Direction,
    normal: &Direction,
    clear_coat_gloss: f32,
) -> f32 {
    if !same_hemisphere(wo, wi, normal) {
        return 0f32;
    }

    let wh = *wi + *wo;
    let wh = normalize(&wh);
    let n_dot_h = dot(&wh, normal).abs();
    let dr = gtr1(n_dot_h, mix(0.1, 0.001, clear_coat_gloss));
    dr * n_dot_h * (4.0 * dot(wo, &wh))
}

pub fn pdf_disney_micro_facet_anisotropic(
    wi: &Direction,
    wo: &Direction,
    normal: &Direction,
    x: &Direction,
    y: &Direction,
    roughness: f32,
    anisotropy: f32,
) -> f32 {
    if !same_hemisphere(wo, wi, normal) {
        return 0f32;
    }

    let wh = *wi + *wo;
    let wh = normalize(&wh);
    let alpha_2 = pow2(roughness);
    let aspect = (1.0 - anisotropy * 0.9).sqrt();
    let alpha_x = (alpha_2 / aspect).max(0.001);
    let alpha_x_2 = pow2(alpha_x);
    let alpha_y = (alpha_2 * aspect).max(0.001);
    let alpha_y_2 = pow2(alpha_y);

    let h_dot_x = dot(&wh, x);
    let h_dot_y = dot(&wh, y);
    let n_dot_h = dot(normal, &wh);

    let denom = h_dot_x * h_dot_x / alpha_x_2 + h_dot_y * h_dot_y / alpha_y_2 + pow2(n_dot_h);
    if denom == 0. {
        return 0f32;
    }

    let pdf_distribution = n_dot_h / (PI * alpha_x * alpha_y * pow2(denom));
    pdf_distribution / (4. * dot(wo, &wh))
}

pub fn pdf_disney(
    wi: &Direction,
    wo: &Direction,
    normal: &Direction,
    x: &Direction,
    y: &Direction,
    roughness: f32,
    anisotropy: f32,
) -> f32 {
    let pdf_diffuse = pdf_lambert(wi, wo, normal);
    let pdf_specular =
        pdf_disney_micro_facet_anisotropic(wi, wo, normal, x, y, roughness, anisotropy);
    let total = pdf_diffuse + pdf_specular;
    total * 0.5;
    pdf_diffuse
}
