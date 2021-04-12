#[derive(Copy, Clone)]
pub struct Vector<const SIZE: usize> {
    data: [f32; SIZE]
}

impl<const SIZE: usize> Vector<SIZE>{
    pub fn new() -> Self {
        Self{data: [0.; SIZE]}
    }
}

pub fn dot<const SIZE: usize>(lhs: &Vector<SIZE>, rhs: &Vector<SIZE>) -> f32 {
    let mut sum = 0.;
    for i in 0..SIZE {
        sum += lhs.data[i] * rhs.data[i]
    }
    
    sum
}

pub fn length<const SIZE: usize>(v: &Vector<SIZE>) -> f32 {
    dot(v, v).sqrt()
}

pub trait XAccessor{
    fn x(&self) -> f32;
}

pub trait YAccessor{
    fn y(&self) -> f32;
}

pub trait ZAccessor{
    fn z(&self) -> f32;
}

pub trait WAccessor{
    fn w(&self) -> f32;
}

impl<const SIZE: usize> XAccessor for Vector<SIZE>{
    fn x(&self) -> f32 {
        self.data[0]
    }
}

impl<const SIZE: usize> YAccessor for Vector<SIZE>{
    fn y(&self) -> f32 {
        self.data[1]
    }
}

impl<const SIZE: usize> ZAccessor for Vector<SIZE>{
    fn z(&self) -> f32 {
        self.data[2]
    }
}

impl<const SIZE: usize> WAccessor for Vector<SIZE>{
    fn w(&self) -> f32 {
        self.data[3]
    }
}

// impl Vector {
//     pub fn new(x: f32, y: f32, z: f32) -> Self {
//         Self { x, y, z }
//     }

//     pub fn one() -> Self {
//         Self {
//             x: 1.,
//             y: 1.,
//             z: 1.,
//         }
//     }

//     pub fn x(&self) -> f32 {
//         self.x
//     }

//     pub fn y(&self) -> f32 {
//         self.y
//     }

//     pub fn z(&self) -> f32 {
//         self.z
//     }
    
//     pub fn dot(&self, other: &Self) -> f32 {
//         self.x * other.x + self.y * other.y + self.z * other.z
//     }
    
//     pub fn length(&self) -> f32 {
//         self.dot(self).sqrt()
//     }
    
//     pub fn normalized(&self) -> Self {
//         *self / self.length()
//     }
    
//     pub fn reflect(&self, normal: &Self) -> Self {
//         *self - *normal * self.dot(normal) * 2.
//     }
// }

impl<const SIZE: usize> std::ops::Add<&Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn add(self, b: &Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] + b.data[i]
        }
        
        result
    }
}

// impl std::ops::Add<f32> for Vec3 {
//     type Output = Self;
//     fn add(self, b: f32) -> Self {
//         Self::new(self.x + b, self.y + b, self.z + b)
//     }
// }

// impl std::ops::AddAssign<&Vec3> for Vec3 {
//     fn add_assign(&mut self, b: &Vec3) {
//         self.x += b.x;
//         self.y += b.y;
//         self.z += b.z;
//     }
// }

// impl std::ops::AddAssign<f32> for Vec3 {
//     fn add_assign(&mut self, b: f32) {
//         self.x += b;
//         self.y += b;
//         self.z += b;
//     }
// }

// impl std::ops::Sub<&Vec3> for Vec3 {
//     type Output = Self;
//     fn sub(self, b: &Vec3) -> Self {
//         Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
//     }
// }

// impl std::ops::Sub<Vec3> for Vec3 {
//     type Output = Self;
//     fn sub(self, b: Vec3) -> Self {
//         Self::new(self.x - b.x, self.y - b.y, self.z - b.z)
//     }
// }

// impl std::ops::Sub<f32> for Vec3 {
//     type Output = Self;
//     fn sub(self, b: f32) -> Self {
//         Self::new(self.x + b, self.y + b, self.z + b)
//     }
// }

// impl std::ops::SubAssign<&Vec3> for Vec3 {
//     fn sub_assign(&mut self, b: &Vec3) {
//         self.x -= b.x;
//         self.y -= b.y;
//         self.z -= b.z;
//     }
// }

// impl std::ops::SubAssign<f32> for Vec3 {
//     fn sub_assign(&mut self, b: f32) {
//         self.x -= b;
//         self.y -= b;
//         self.z -= b;
//     }
// }

// impl std::ops::Mul<&Vec3> for Vec3 {
//     type Output = Self;
//     fn mul(self, b: &Vec3) -> Self {
//         Self::new(self.x * b.x, self.y * b.y, self.z * b.z)
//     }
// }

// impl std::ops::Mul<Vec3> for Vec3 {
//     type Output = Self;
//     fn mul(self, b: Vec3) -> Self {
//         Self::new(self.x * b.x, self.y * b.y, self.z * b.z)
//     }
// }

// impl std::ops::Mul<f32> for Vec3 {
//     type Output = Self;
//     fn mul(self, b: f32) -> Self {
//         Self::new(self.x * b, self.y * b, self.z * b)
//     }
// }

// impl std::ops::MulAssign<&Vec3> for Vec3 {
//     fn mul_assign(&mut self, b: &Vec3) {
//         self.x *= b.x;
//         self.y *= b.y;
//         self.z *= b.z;
//     }
// }

// impl std::ops::MulAssign<f32> for Vec3 {
//     fn mul_assign(&mut self, b: f32) {
//         self.x *= b;
//         self.y *= b;
//         self.z *= b;
//     }
// }

// impl std::ops::Div<&Vec3> for Vec3 {
//     type Output = Self;
//     fn div(self, b: &Vec3) -> Self {
//         Self::new(self.x / b.x, self.y / b.y, self.z / b.z)
//     }
// }

// impl std::ops::Div<f32> for Vec3 {
//     type Output = Self;
//     fn div(self, b: f32) -> Self {
//         Self::new(self.x / b, self.y / b, self.z / b)
//     }
// }

// impl std::ops::DivAssign<&Vec3> for Vec3 {
//     fn div_assign(&mut self, b: &Vec3) {
//         self.x /= b.x;
//         self.y /= b.y;
//         self.z /= b.z;
//     }
// }

// impl std::ops::DivAssign<f32> for Vec3 {
//     fn div_assign(&mut self, b: f32) {
//         self.x /= b;
//         self.y /= b;
//         self.z /= b;
//     }
// }

fn main() {
    
}
