use slotmap::DefaultKey;

use crate::disney_brdf_sample::sample_disney_bsdf;
use crate::disney_brdf_sample::sample_disney_micro_facet_anisotropic;
use crate::types::Direction;

use super::brdf::*;
use super::disney_brdf_evaluate::*;
use super::material::*;
use super::onb::*;
use super::rand;
use super::ray::Ray;
use super::resources::Resources;
use super::types::Color;
use super::vec::*;
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

    fn scatter(&self, _: &Resources, hit_record: &HitRecord) -> Bounce {
        let onb = OrthoNormalBasis::from_w(&hit_record.normal);
        let dir = onb.local(&rand::cosine());
        let cos_theta = saturate(dot(&dir, &hit_record.normal));

        let new_origin = hit_record.position() + hit_record.normal * 0.05;
        let ray = Ray::new(&new_origin, &dir);
        Bounce::new(&ray, cos_theta / PI)
    }

    fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
        let cos_theta = saturate(dot(&hit_record.bounce.ray.direction(), &hit_record.normal));
        resources
            .texture(self.albedo)
            .sample(resources, &hit_record.uv, &hit_record.position())
            * cos_theta
            / PI
    }
}

pub struct MirrorMaterial {
    albedo: DefaultKey,
}

impl Material for MirrorMaterial {
    fn uid(&self) -> usize {
        2
    }

    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let new_origin = hit_record.position() + hit_record.normal * 0.05;
        let out_dir = reflect(&hit_record.ray_direction(), &hit_record.normal);
        let ray = Ray::new(&new_origin, &out_dir);
        let color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );
        Bounce::new(&ray, 1.0)
    }

    fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
        resources
            .texture(self.albedo)
            .sample(resources, &hit_record.uv, &hit_record.position())
    }
}

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

impl Material for TranslucentMaterial {
    fn uid(&self) -> usize {
        3
    }

    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let ratio = if hit_record.front_facing {
            1.0 / self.ior
        } else {
            self.ior
        };

        let cos_theta = dot(&(-hit_record.ray_direction()), &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();

        let cannot_refract = ratio * sin_theta > 1.0;
        let f = rand::float();
        let wo = if cannot_refract || fresnel_schlick_reflectance(cos_theta, ratio) > f {
            reflect(hit_record.ray_direction(), &hit_record.normal)
        } else {
            refract_glsl(hit_record.ray_direction(), &hit_record.normal, ratio)
        };

        let new_origin = hit_record.position() + wo * 0.05;

        let ray = Ray::new(&new_origin, &wo);
        let color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );

        Bounce::new(&ray, 1.0)
    }

    fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
        resources
            .texture(self.albedo)
            .sample(resources, &hit_record.uv, &hit_record.position())
    }
}

pub struct PBRMaterial {
    pub albedo: DefaultKey,
    pub roughness: DefaultKey,
    pub metal: DefaultKey,
    pub emission: DefaultKey,
    pub anisotropy: f32,
}

impl PBRMaterial {
    pub fn new(
        albedo: DefaultKey,
        roughness: DefaultKey,
        metal: DefaultKey,
        emission: DefaultKey,
    ) -> Self {
        Self {
            albedo,
            roughness,
            metal,
            emission,
            anisotropy: 0.0,
        }
    }
}

impl Material for PBRMaterial {
    fn uid(&self) -> usize {
        4
    }

    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let roughness = resources
            .texture(self.roughness)
            .sample(resources, &hit_record.uv, &hit_record.position())
            .x();

        let (x, y) = direction_of_anisotropicity(&hit_record.normal);

        let (wi, pdf) = sample_disney_bsdf(
            &-hit_record.ray_direction(),
            &hit_record.normal,
            &x,
            &y,
            roughness,
            self.anisotropy,
        );

        let ray = Ray::new(&hit_record.position(), &wi);
        Bounce::new(&ray, pdf)
    }

    fn evaluate(&self, resources: &Resources, hit_record: &HitRecord) -> Color {
        let base_color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );

        let roughness = resources
            .texture(self.roughness)
            .sample(resources, &hit_record.uv, &hit_record.position())
            .x()
            .min(0.25);

        let metal = resources
            .texture(self.metal)
            .sample(resources, &hit_record.uv, &hit_record.position())
            .x();
        let sheen = 0.0;
        let sheen_tint = 0.0;
        let clear_coat = 0.0;
        let clear_coat_boost = 0.0;
        let clear_coat_gloss = 0.0;
        let sub_surface = 0.0;

        let bounce = &hit_record.bounce;
        let (x, y) = direction_of_anisotropicity(&hit_record.normal);
        evaluate_disney_bsdf(
            &bounce.ray.dir,
            &-hit_record.ray_direction(),
            &hit_record.normal,
            &x,
            &y,
            &base_color,
            roughness,
            metal,
            sheen,
            sheen_tint,
            clear_coat,
            clear_coat_boost,
            clear_coat_gloss,
            self.anisotropy,
            sub_surface,
        )
    }

    fn emit(&self, _: &Resources, _hit_record: &HitRecord) -> Color {
        Color::new()
    }
}
