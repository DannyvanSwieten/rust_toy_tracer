pub mod acceleration_structure;
pub mod bounding_box;
pub mod brdf;
pub mod camera;
pub mod cpu_tracer;
pub mod default_camera;
pub mod default_ray_generation_shader;
pub mod disney_brdf_evaluate;
pub mod disney_brdf_pdf;
pub mod disney_brdf_sample;
pub mod hittable;
pub mod intersection;
pub mod light;
pub mod mat;
pub mod material;
pub mod materials;
pub mod math_utils;
pub mod onb;
pub mod rand;
pub mod ray;
pub mod raytracer;
pub mod resources;
pub mod scene;
pub mod texture;
pub mod types;
pub mod vec;
pub mod vec_add;
pub mod vec_div;
pub mod vec_mul;
pub mod vec_sub;

use acceleration_structure::*;
use cpu_tracer::*;
use default_camera::DefaultCamera;
use default_ray_generation_shader::RayGenerator;
use hittable::*;
use light::{DirectionalLight, Lights};
use materials::*;
use raytracer::*;
use resources::Resources;
use scene::*;
use texture::*;
use types::*;
use vec::*;

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.
}

fn main() {
    let obj_file = "./assets/stanford-bunny.obj";
    let (models, _) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to load file");

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut tex_coords = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (_, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        indices.extend(mesh.indices.iter());

        for v in (0..mesh.positions.len()).step_by(3) {
            positions.push(Position::from_values([
                mesh.positions[v],
                mesh.positions[v + 1],
                mesh.positions[v + 2],
            ]))
        }

        for n in (0..mesh.normals.len()).step_by(3) {
            normals.push(Normal::from_values([
                mesh.normals[n],
                mesh.normals[n + 1],
                mesh.normals[n + 2],
            ]))
        }

        for t in (0..mesh.texcoords.len()).step_by(2) {
            tex_coords.push(TextureCoordinate::from_values([
                mesh.texcoords[t],
                mesh.texcoords[t + 1],
            ]))
        }
    }

    let mut resources = Resources::default();
    let sphere = resources.add_hittable(Sphere::new(1.0, &Position::default()));
    let teapot = resources.add_hittable(TriangleMesh::new(positions, normals, tex_coords, indices));

    let grey_texture = resources.add_texture(SolidColorTexture::new(&Color::from_values([
        0.05, 0.3, 0.25,
    ])));
    let white_texture = resources.add_texture(SolidColorTexture::new(&Color::splat(1.0)));
    let black_texture = resources.add_texture(SolidColorTexture::new(&Color::new()));
    let checker_texture =
        resources.add_texture(CheckerTexture::new(white_texture, black_texture, 3.0));

    let purple_texture =
        resources.add_texture(SolidColorTexture::new(&Color::from_values([0.4, 0.1, 0.4])));

    let pbr = resources.add_material(PBRMaterial::new(
        checker_texture,
        grey_texture,
        black_texture,
        black_texture,
        1.0,
        0.0,
        0.5,
    ));

    let pbr2 = resources.add_material(PBRMaterial::new(
        purple_texture,
        grey_texture,
        black_texture,
        black_texture,
        1.1,
        1.0,
        0.5,
    ));

    let pbr3 = resources.add_material(PBRMaterial::new(
        white_texture,
        white_texture,
        black_texture,
        black_texture,
        1.5,
        0.0,
        0.5,
    ));

    let mut instances = Vec::new();
    // Floor
    instances.push(
        Instance::new(sphere, 0, pbr, true)
            .with_position(0., -100., 0.)
            .with_scale(100., 100., 100.),
    );

    instances.push(
        Instance::new(teapot, 1, pbr2, false)
            .with_position(0.0, -2., 0.)
            .with_uniform_scale(50.),
    );

    instances.push(
        Instance::new(teapot, 2, pbr, true)
            .with_position(5.0, -2., 4.)
            .with_uniform_scale(50.),
    );

    instances.push(
        Instance::new(teapot, 3, pbr3, true)
            .with_position(-5.0, -2., -3.)
            .with_uniform_scale(50.),
    );

    let ac = TopLevelAccelerationStructure::new(&resources.hittables(), &instances);
    let width = 1920;
    let height = 1080;
    let origin = &Position::from_values([3., 4., 15.]);
    let look_at = &Direction::from_values([0., 3., 0.]);
    let camera = DefaultCamera::new(
        &origin,
        &look_at,
        width as f32 / height as f32,
        45.,
        0.15,
        distance(&look_at, &origin),
    );

    let mut lights = Lights::new();
    // lights.add(DirectionalLight::new(Position::from_values([-1., 1., 1.])));

    let tracer = CPUTracer::new(RayGenerator { camera });
    tracer.trace(1024, 32, width, height, &ac, &lights, &resources);
}
