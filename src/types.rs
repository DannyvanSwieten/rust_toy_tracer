use super::mat::*;
use super::vec::*;

pub type Vec3 = Vector<3>;
pub type Vec2 = Vector<2>;
pub type Mat3x4 = Matrix<3, 4>;

pub type Color = Vec3;
pub type Normal = Vec3;
pub type Position = Vec3;
pub type Direction = Vec3;
pub type Barycentrics = Vec2;
//pub type FragCoord = IVec2;
pub type TextureCoordinate = Vec2;
pub type Transform = Mat3x4;
