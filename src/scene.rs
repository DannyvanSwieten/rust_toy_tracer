use slotmap::DefaultKey;

use super::types::*;

#[derive(Copy, Clone)]
pub struct Instance {
    pub geometry_index: DefaultKey,
    pub instance_id: u32,
    pub hit_shader_id: u32,
    pub material_id: DefaultKey,
    pub transform: Transform,
    pub cull: bool,
}

impl Instance {
    pub fn new(
        geometry_index: DefaultKey,
        instance_id: u32,
        material_id: DefaultKey,
        cull: bool,
    ) -> Self {
        Self {
            geometry_index,
            instance_id,
            hit_shader_id: 0,
            material_id,
            transform: Transform::new(),
            cull,
        }
    }

    pub fn with_position(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform.colums[0][3] = x;
        self.transform.colums[1][3] = y;
        self.transform.colums[2][3] = z;
        self
    }

    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform.colums[0][0] = x;
        self.transform.colums[1][1] = y;
        self.transform.colums[2][2] = z;
        self
    }

    pub fn with_uniform_scale(mut self, s: f32) -> Self {
        self.transform.colums[0][0] = s;
        self.transform.colums[1][1] = s;
        self.transform.colums[2][2] = s;
        self
    }
}
