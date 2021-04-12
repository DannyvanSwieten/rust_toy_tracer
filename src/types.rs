use super::nalgebra_glm::Mat3x4;
use super::nalgebra_glm::Vec2;
use super::nalgebra_glm::IVec2;
use super::nalgebra_glm::Vec3;

pub type Color = Vec3;
pub type Normal = Vec3;
pub type Position = Vec3;
pub type Direction = Vec3;
pub type Barycentrics = Vec2;
pub type FragCoord = IVec2;
pub type TextureCoordinate = Vec2;
pub type Transform = Mat3x4;
