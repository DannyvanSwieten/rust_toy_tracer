use image::RgbaImage;
use slotmap::{DefaultKey, SlotMap};

use super::hittable::Hittable;
use super::material::Material;
use super::texture::Texture;

#[derive(Default)]
pub struct Resources {
    images: SlotMap<DefaultKey, RgbaImage>,
    textures: SlotMap<DefaultKey, Box<dyn Texture>>,
    materials: SlotMap<DefaultKey, Box<dyn Material>>,
    hittables: SlotMap<DefaultKey, Box<dyn Hittable>>,
}

impl Resources {
    pub fn add_texture<T>(&mut self, t: T) -> DefaultKey
    where
        T: Texture + 'static,
    {
        self.textures.insert(Box::new(t))
    }

    pub fn texture(&self, id: DefaultKey) -> &dyn Texture {
        self.textures[id].as_ref()
    }

    pub fn add_material<M>(&mut self, m: M) -> DefaultKey
    where
        M: Material + 'static,
    {
        self.materials.insert(Box::new(m))
    }

    pub fn add_image(&mut self, image: RgbaImage) -> DefaultKey {
        self.images.insert(image)
    }

    pub fn material(&self, id: DefaultKey) -> &dyn Material {
        self.materials[id].as_ref()
    }

    pub fn add_hittable<H>(&mut self, h: H) -> DefaultKey
    where
        H: Hittable + 'static,
    {
        self.hittables.insert(Box::new(h))
    }

    pub fn hittables(&self) -> &SlotMap<DefaultKey, Box<dyn Hittable>> {
        &self.hittables
    }

    pub fn hittable(&self, id: DefaultKey) -> &dyn Hittable {
        self.hittables[id].as_ref()
    }
}

unsafe impl Send for Resources {}
unsafe impl Sync for Resources {}
