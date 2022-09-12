use crate::brdf::saturate;
use crate::math_utils::{mix, mix_vec3, pow2, same_hemisphere};
use crate::onb::OrthoNormalBasis;
use crate::types::{Color, Direction, Vec3};

use super::rand;
use super::vec::*;
use std::f32::consts::PI;

pub fn schlick_weight(cos_theta: f32) -> f32 {
    let m = saturate(1. - cos_theta);
    return (m * m) * (m * m) * m;
}

pub fn gtr1(n_dot_h: f32, a: f32) -> f32 {
    if a >= 1.0 {
        return 1. / PI;
    }

    let a2 = a * a;
    let a21 = a2 - 1.0;
    let t = 1. + a21 * n_dot_h * n_dot_h;
    a21 / (PI * a2.ln() * t)
}

pub fn gtr2(n_dot_h: f32, a: f32) -> f32 {
    let a2 = a * a;
    let a21 = a2 - 1.0;
    let t = 1. + a21 * n_dot_h * n_dot_h;
    a2 / (PI * t * t)
}

pub fn gtr2_anisotropic(n_dot_h: f32, h_dot_x: f32, h_dot_y: f32, ax: f32, ay: f32) -> f32 {
    return 1. / (PI * ax * ay * pow2(pow2(h_dot_x / ax) + pow2(h_dot_y / ay) + pow2(n_dot_h)));
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

pub fn smith_g_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let a = pow2(roughness);
    let b = pow2(n_dot_v);
    1.0 / n_dot_v.abs() + (a + b - a * b).sqrt().max(0.001)
}

pub fn smith_g_ggx_aniso(n_dot_v: f32, v_dot_x: f32, v_dot_y: f32, ax: f32, ay: f32) -> f32 {
    let t1 = pow2(v_dot_x * ax);
    let t2 = pow2(v_dot_y * ay);
    let t3 = pow2(n_dot_v);
    let t = (t1 + t2 + t3).sqrt() + n_dot_v;
    1. / t
}

pub fn direction_of_anisotropicity(normal: &Direction) -> (Direction, Direction) {
    let tangent = cross(normal, &Direction::from_values([1., 0., 1.]));
    let binormal = normalize(&cross(normal, &tangent));
    let tangent = normalize(&cross(normal, &binormal));
    (tangent, binormal)
}

pub fn evaluate_disney_diffuse(
    n_dot_l: f32,
    n_dot_v: f32,
    l_dot_h: f32,
    base_color: &Color,
    roughness: f32,
) -> Color {
    let fl = schlick_weight(n_dot_l);
    let fv = schlick_weight(n_dot_v);

    let fd90 = 0.5 + 2. * pow2(l_dot_h) * roughness;
    let fd = mix(1.0, fd90, fl) * mix(1.0, fd90, fv);

    (1.0 / PI) * fd * base_color
}

pub fn evaluate_disney_subsurface(
    n_dot_l: f32,
    n_dot_v: f32,
    l_dot_h: f32,
    base_color: &Color,
    roughness: f32,
) -> Color {
    let fl = schlick_weight(n_dot_l);
    let fv = schlick_weight(n_dot_v);

    let fss90 = pow2(l_dot_h) * roughness;
    let fss = mix(1.0, fss90, fl) * mix(1.0, fss90, fv);
    let ss = 1.25 * (fss * (1. / (n_dot_l + n_dot_v) - 0.5) + 0.5);

    (1. / PI) * ss * base_color
}

pub fn evaluate_disney_clear_coat(
    n_dot_l: f32,
    n_dot_v: f32,
    n_dot_h: f32,
    l_dot_h: f32,
    clear_coat: f32,
    clear_coat_gloss: f32,
    clear_coat_boost: f32,
) -> f32 {
    let gloss = mix(0.1, 0.001, clear_coat_gloss);
    let dr = gtr1(n_dot_h.abs(), gloss);
    let fh = schlick_weight(l_dot_h);
    let fr = mix(0.04, 1.0, fh);
    let gr = smith_g_ggx(n_dot_l, 0.25) * smith_g_ggx(n_dot_v, 0.25);
    clear_coat_boost * clear_coat * fr * gr * dr
}

pub fn evaluate_disney_sheen(
    l_dot_h: f32,
    base_color: &Color,
    sheen: f32,
    sheen_tint: f32,
) -> Color {
    let fh = schlick_weight(l_dot_h);
    let c_lum = 0.3 * base_color.r() + 0.6 * base_color.g() * 0.1 * base_color.b();
    let c_tint = if c_lum > 0. {
        *base_color / c_lum
    } else {
        Color::ones()
    };

    let c_sheen = mix_vec3(&Color::ones(), &c_tint, sheen_tint);
    fh * sheen * c_sheen
}

pub fn evaluate_disney_anisotropic_specular(
    n_dot_l: f32,
    n_dot_v: f32,
    n_dot_h: f32,
    l_dot_h: f32,
    l: &Direction,
    v: &Direction,
    h: &Direction,
    x: &Direction,
    y: &Direction,
    base_color: &Color,
    roughness: f32,
    metal: f32,
    anisotropy: f32,
) -> Color {
    let cdlum = 0.3 * base_color.x() + 0.6 * base_color.y() + 0.1 * base_color.z(); // luminance approx.

    let ctint = if cdlum > 0f32 {
        base_color * 1.0 / cdlum
    } else {
        Vec3::from_values([1., 1., 1.])
    }; // normalize lum. to isolate hue+sat
    let specular = 0.0;
    let specular_tint = 0.0;
    let tint = mix_vec3(&Vec3::from_values([1., 1., 1.]), &ctint, specular_tint);
    let cspec0 = mix_vec3(&(specular * 0.08 * tint), base_color, metal);

    let roughness_2 = roughness * roughness;

    let aspect = (1. - anisotropy * 0.9).sqrt();
    let ax = (roughness_2 / aspect).max(0.001);
    let ay = (roughness_2 * aspect).max(0.001);
    let ds = gtr2_anisotropic(n_dot_h, dot(h, x), dot(h, y), ax, ay);
    let fh = schlick_weight(l_dot_h);
    let fs = mix_vec3(&cspec0, &Vec3::from_values([1., 1., 1.]), fh);
    let gs = smith_g_ggx_aniso(n_dot_l, dot(l, x), dot(l, y), ax, ay)
        * smith_g_ggx_aniso(n_dot_v, dot(v, x), dot(v, y), ax, ay);

    return gs * fs * ds;
}

pub fn evaluate_disney_isotropic_specular() {}

pub fn evaluate_disney_micro_facet_anisotropic(
    wi: &Direction,
    wo: &Direction,
    normal: &Direction,
    x: &Direction,
    y: &Direction,
    base_color: &Color,
    roughness: f32,
    metal: f32,
    anisotropy: f32,
) -> Color {
    if !same_hemisphere(wo, wi, normal) {
        return Color::new();
    }

    let n_dot_l = dot(normal, wo);
    let n_dot_v = dot(normal, wi);

    if n_dot_l < 0. || n_dot_v < 0. {
        return Color::new();
    };

    let h = *wo + *wi;
    let h = normalize(&h);
    let n_dot_h = dot(normal, &h);
    let l_dot_h = dot(wo, &h);

    let diffuse = evaluate_disney_diffuse(n_dot_l, n_dot_v, l_dot_h, base_color, roughness);
    let specular = evaluate_disney_anisotropic_specular(
        n_dot_l, n_dot_v, n_dot_h, l_dot_h, wi, wo, &h, x, y, base_color, roughness, metal,
        anisotropy,
    );

    diffuse * (1.0 - metal) + specular
}

pub fn evaluate_disney_bsdf(
    wi: &Direction,
    wo: &Direction,
    normal: &Direction,
    x: &Direction,
    y: &Direction,
    base_color: &Color,
    roughness: f32,
    metal: f32,
    sheen: f32,
    sheen_tint: f32,
    clear_coat: f32,
    clear_coat_boost: f32,
    clear_coat_gloss: f32,
    anisotropy: f32,
    sub_surface: f32,
) -> Color {
    if !same_hemisphere(wo, wi, normal) {
        return Color::new();
    }

    let n_dot_l = dot(&normal, wo);
    let n_dot_v = dot(&normal, wi);

    if n_dot_l < 0.0 || n_dot_v < 0.0 {
        return Color::new();
    }
    let h = *wi + *wo;
    let h = normalize(&h);
    let n_dot_h = dot(normal, &h);
    let l_dot_h = dot(wo, &h);

    let diffuse_brdf = evaluate_disney_diffuse(n_dot_l, n_dot_v, l_dot_h, base_color, roughness);
    let sub_surface_brdf =
        evaluate_disney_subsurface(n_dot_l, n_dot_v, l_dot_h, base_color, roughness);
    let gloss_brdf = evaluate_disney_anisotropic_specular(
        n_dot_l, n_dot_v, n_dot_h, l_dot_h, wi, wo, &h, x, y, base_color, roughness, metal,
        anisotropy,
    );
    let clear_coat_brdf = evaluate_disney_clear_coat(
        n_dot_l,
        n_dot_v,
        n_dot_h,
        l_dot_h,
        clear_coat,
        clear_coat_gloss,
        clear_coat_boost,
    );
    let sheen_brdf = evaluate_disney_sheen(l_dot_h, base_color, sheen, sheen_tint);

    let kd = mix_vec3(&diffuse_brdf, &sub_surface_brdf, sub_surface) + sheen_brdf;
    let kd = kd * (1.0 - metal);
    let theta = dot(normal, wi).max(0.001);
    (kd) // + gloss_brdf + clear_coat_brdf) * theta
}
