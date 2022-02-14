use serde::{Deserialize, Serialize};

use super::hittable::Hittable;
use super::material::Material;
use super::texture::Texture;

#[derive(Default)]
pub struct Resources {
    textures: Vec<Box<dyn Texture>>,
    materials: Vec<Box<dyn Material>>,
    hittables: Vec<Box<dyn Hittable>>,
}

impl Resources {
    pub fn add_texture<T>(&mut self, t: T)
    where
        T: Texture + 'static,
    {
        self.textures.push(Box::new(t))
    }

    pub fn texture(&self, id: u32) -> &dyn Texture {
        self.textures[id as usize].as_ref()
    }

    pub fn add_material<M>(&mut self, m: M)
    where
        M: Material + 'static,
    {
        self.materials.push(Box::new(m))
    }

    pub fn material(&self, id: usize) -> &dyn Material {
        self.materials[id].as_ref()
    }

    pub fn add_hittable<H>(&mut self, h: H)
    where
        H: Hittable + 'static,
    {
        self.hittables.push(Box::new(h))
    }

    pub fn hittables(&self) -> &Vec<Box<dyn Hittable>> {
        &self.hittables
    }

    pub fn hittable(&self, id: usize) -> &dyn Hittable {
        self.hittables[id].as_ref()
    }
}

unsafe impl Send for Resources {}
unsafe impl Sync for Resources {}
