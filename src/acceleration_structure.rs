use super::scene::*;

pub struct AccelerationStructure {
    left: Box<dyn Hittable + Send + Sync>,
    right: Box<dyn Hittable + Send + Sync>,
}

impl AccelerationStructure {
    pub fn new(scene: &Scene){
         
    }
}