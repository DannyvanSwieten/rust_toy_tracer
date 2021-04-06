use super::types::*;

pub struct Intersection {
    pub position: Position,
    pub in_direction: Direction,
    pub t: f32,
    pub normal: Normal,
    pub uv: TextureCoordinate,
    pub object_id: u32,
    pub instance_id: u32,
    pub primitive_id: u32,
    pub material_id: u32,
    pub barycentrics: Barycentrics,
}

impl Intersection {
    pub fn new(
        position: &Position,
        in_direction: &Direction,
        t: f32,
        normal: &Normal,
        uv: &TextureCoordinate,
        object_id: u32,
        instance_id: u32,
        primitive_id: u32,
        material_id: u32,
        barycentrics: &Barycentrics,
    ) -> Self {
        Self {
            position: *position,
            in_direction: *in_direction,
            t,
            uv: *uv,
            normal: *normal,
            object_id,
            instance_id,
            primitive_id,
            material_id,
            barycentrics: *barycentrics,
        }
    }
}
