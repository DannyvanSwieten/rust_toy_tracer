use slotmap::DefaultKey;

use super::brdf::*;
use super::disney_brdf_evaluate::*;
use super::material::*;
use super::onb::*;
use super::rand;
use super::ray::Ray;
use super::resources::Resources;
use super::types::Color;
use super::vec::*;
use crate::disney_brdf_sample::sample_disney_bsdf;
use crate::disney_brdf_sample::sample_disney_diffuse;
use crate::rand::float;
use crate::rand::sphere;
use std::f32::consts::PI;
pub struct DiffuseMaterial {
    albedo: DefaultKey,
}

impl DiffuseMaterial {
    pub fn new(albedo: DefaultKey) -> Self {
        Self { albedo }
    }
}

impl Material for DiffuseMaterial {
    fn uid(&self) -> usize {
        1
    }

    fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let onb = OrthoNormalBasis::from_w(&hit_record.normal);
        let dir = onb.local(&rand::cosine());
        let cos_theta = saturate(dot(&dir, &hit_record.normal));

        let color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        ) / PI;

        Bounce::new(&dir, &color)
    }
}

pub struct MirrorMaterial {
    albedo: DefaultKey,
}

// impl Material for MirrorMaterial {
//     fn uid(&self) -> usize {
//         2
//     }

//     fn scatter(&self, _: &Resources, hit_record: &HitRecord) -> Bounce {
//         let new_origin = hit_record.position() + hit_record.normal * 0.05;
//         let out_dir = reflect(&hit_record.ray_direction(), &hit_record.normal);
//         let ray = Ray::new(&new_origin, &out_dir);
//         Bounce::new(&ray, 1.0)
//     }

//     fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
//         resources
//             .texture(self.albedo)
//             .sample(resources, &hit_record.uv, &hit_record.position())
//     }
// }

impl MirrorMaterial {
    pub fn new(albedo: DefaultKey) -> Self {
        Self { albedo }
    }
}

pub struct TranslucentMaterial {
    albedo: DefaultKey,
    ior: f32,
}

impl TranslucentMaterial {
    pub fn new(albedo: DefaultKey, ior: f32) -> Self {
        Self { albedo, ior }
    }
}

// impl Material for TranslucentMaterial {
//     fn uid(&self) -> usize {
//         3
//     }

//     fn scatter(&self, _: &Resources, hit_record: &HitRecord) -> Bounce {
//         let ratio = if hit_record.front_facing {
//             1.0 / self.ior
//         } else {
//             self.ior
//         };

//         let cos_theta = dot(&(-hit_record.ray_direction()), &hit_record.normal).min(1.0);
//         let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();

//         let cannot_refract = ratio * sin_theta > 1.0;
//         let f = rand::float();
//         let wo = if cannot_refract || fresnel_schlick_reflectance(cos_theta, ratio) > f {
//             reflect(hit_record.ray_direction(), &hit_record.normal)
//         } else {
//             refract_glsl(hit_record.ray_direction(), &hit_record.normal, ratio)
//         };

//         let new_origin = hit_record.position() + wo * 0.05;

//         let ray = Ray::new(&new_origin, &wo);
//         Bounce::new(&ray, 1.0)
//     }

//     fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
//         resources
//             .texture(self.albedo)
//             .sample(resources, &hit_record.uv, &hit_record.position())
//     }
// }

pub struct PBRMaterial {
    pub albedo: DefaultKey,
    pub roughness: DefaultKey,
    pub metal: DefaultKey,
    pub emission: DefaultKey,
    pub ior: f32,
    pub transmission: f32,
    pub fresnel_reflectance: f32,
}

impl PBRMaterial {
    pub fn new(
        albedo: DefaultKey,
        roughness: DefaultKey,
        metal: DefaultKey,
        emission: DefaultKey,
        ior: f32,
        transmission: f32,
        fresnel_reflectance: f32,
    ) -> Self {
        Self {
            albedo,
            roughness,
            metal,
            emission,
            ior,
            transmission,
            fresnel_reflectance,
        }
    }
}

impl Material for PBRMaterial {
    fn uid(&self) -> usize {
        4
    }

    fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let base_color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );

        let roughness = resources
            .texture(self.roughness)
            .sample(resources, &hit_record.uv, &hit_record.position())
            .x();

        let metal = resources
            .texture(self.metal)
            .sample(resources, &hit_record.uv, &hit_record.position())
            .x();

        let v = -hit_record.ray_direction();
        let r_brdf = float();
        let (wi, color) = if r_brdf < 0.5 {
            if 2.0 * r_brdf < self.transmission {
                sample_microfacet_transmission_brdf(
                    &v,
                    &hit_record.normal,
                    &base_color,
                    metal,
                    roughness,
                    self.fresnel_reflectance,
                    self.ior,
                )
            } else {
                sample_micro_facet_isotropic_specular_brdf(
                    &v,
                    &hit_record.normal,
                    &base_color,
                    metal,
                    roughness,
                    self.fresnel_reflectance,
                )
            }
        } else {
            sample_diffuse_brdf(&v, &base_color, metal, self.fresnel_reflectance)
        };

        Bounce::new(&wi, &(color * 2.0))
    }

    fn emit(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
        resources
            .texture(self.emission)
            .sample(resources, &hit_record.uv, &hit_record.position())
    }
}
