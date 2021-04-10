use glm::Matrix4x3;
use glm::Vector2;
use glm::Vector3;

pub type Color = Vector3<f32>;
pub type Normal = Vector3<f32>;
pub type Position = Vector3<f32>;
pub type Direction = Vector3<f32>;
pub type Barycentrics = Vector2<f32>;
pub type FragCoord = Vector2<u32>;
pub type TextureCoordinate = Vector2<f32>;
pub type Transform = Matrix4x3<f32>;
