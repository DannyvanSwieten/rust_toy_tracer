use super::brdf::*;
use super::material::*;
use super::onb::*;
use super::rand;
use super::ray::Ray;
use super::resources::Resources;
use super::types::Color;
use super::vec::*;
use std::f32::consts::PI;
pub struct DiffuseMaterial {
    albedo: u32,
}

impl DiffuseMaterial {
    pub fn new(albedo: u32) -> Self {
        Self { albedo }
    }
}

impl Material for DiffuseMaterial {
    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let onb = OrthoNormalBasis::from_w(&hit_record.normal);
        let dir = onb.local(&rand::cosine());
        let cos_theta = saturate(dot(&dir, &hit_record.normal));

        let new_origin = hit_record.position() + hit_record.normal * 0.05;
        let ray = Ray::new(&new_origin, &dir);
        let color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        ) * cos_theta
            / PI;
        Bounce::new(&ray, &color, cos_theta / PI)
    }

    fn emit(&self, _: &Resources, _hit_record: &HitRecord) -> Option<Color> {
        None
    }

    fn uid(&self) -> usize {
        1
    }
}

pub struct MirrorMaterial {
    albedo: u32,
}

impl Material for MirrorMaterial {
    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let new_origin = hit_record.position() + hit_record.normal * 0.05;
        let out_dir = reflect(&hit_record.ray_direction(), &hit_record.normal);
        let ray = Ray::new(&new_origin, &out_dir);
        let color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );
        Bounce::new(&ray, &color, 1.0)
    }

    fn uid(&self) -> usize {
        2
    }
}

impl MirrorMaterial {
    pub fn new(albedo: u32) -> Self {
        Self { albedo }
    }
}

pub struct TranslucentMaterial {
    albedo: u32,
    ior: f32,
}

impl TranslucentMaterial {
    pub fn new(albedo: u32, ior: f32) -> Self {
        Self { albedo, ior }
    }
}

impl Material for TranslucentMaterial {
    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let ratio = if hit_record.front_facing {
            1.0 / self.ior
        } else {
            self.ior
        };

        let cos_theta = dot(&(-hit_record.ray_direction()), &hit_record.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ratio * sin_theta > 1.0;
        let f = rand::float();
        let wo = if cannot_refract || fresnel_schlick_refraction(cos_theta, ratio) > f {
            reflect(hit_record.ray_direction(), &hit_record.normal)
        } else {
            refract(hit_record.ray_direction(), &hit_record.normal, ratio)
        };

        let new_origin = hit_record.position() + hit_record.normal * 0.05;

        let ray = Ray::new(&new_origin, &wo);
        let color = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );

        Bounce::new(&ray, &color, 1.0)
    }

    fn emit(&self, resources: &Resources, _hit_record: &HitRecord) -> Option<Color> {
        None
    }

    fn uid(&self) -> usize {
        3
    }
}

pub struct PBRMaterial {
    pub roughness: u32,
    pub metal: u32,
    pub albedo: u32,
    pub emission: u32,
    diffuse_material: DiffuseMaterial,
}

impl PBRMaterial {
    pub fn new() -> Self {
        Self {
            roughness: 0,
            metal: 0,
            albedo: 0,
            emission: 0,
            diffuse_material: DiffuseMaterial::new(0),
        }
    }
}

impl Material for PBRMaterial {
    fn scatter(&self, resources: &Resources, hit_record: &HitRecord) -> Bounce {
        let roughness = resources
            .texture(self.roughness)
            .sample(resources, &hit_record.uv, &hit_record.position())
            .x();
        let albedo = resources.texture(self.albedo).sample(
            resources,
            &hit_record.uv,
            &hit_record.position(),
        );

        let roughness = 1.0;

        if rand::float() < roughness {
            let mut b = self.diffuse_material.scatter(resources, hit_record);
            b.pdf /= roughness.max(0.000001);
            b
        } else {
            // Create half-vector based on surface normal
            let h = ggx_micro_facet_normal(roughness, &hit_record.normal);

            // reflect incoming ray over half-vector
            let l = reflect(hit_record.ray_direction(), &h);
            let n_dot_v = saturate(dot(&hit_record.normal, &-hit_record.ray_direction()));
            let n_dot_l = saturate(dot(&hit_record.normal, &l));
            let n_dot_h = saturate(dot(&hit_record.normal, &h));
            let l_dot_h = saturate(dot(&l, &h));

            let d = ggx_normal_distribution(n_dot_h, roughness);
            let g = schlick_masking_term(n_dot_l, n_dot_v, roughness);
            let f = schlick_fresnel(&albedo, l_dot_h);

            let ggx = d * g * f / ((4.0 * n_dot_l * n_dot_v) + 0.000001);
            let pdf = d * n_dot_h / ((4.0 * l_dot_h) + 0.000001);

            let p = hit_record.position();
            let radiance = ggx;

            let ray = Ray::new(&p, &l);
            Bounce::new(&ray, &radiance, pdf * (1.0 - roughness.max(0.000001)))
        }
    }

    fn emit(&self, resources: &Resources, _hit_record: &HitRecord) -> Option<crate::types::Color> {
        None
    }

    fn uid(&self) -> usize {
        4
    }
}
