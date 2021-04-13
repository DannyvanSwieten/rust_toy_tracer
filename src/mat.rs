use super::types::*;
use super::vec::*;

#[derive(Copy, Clone)]
pub struct Matrix<const COLUMS: usize, const ROWS: usize> {
    pub colums: [Vector<ROWS>; COLUMS],
}

impl<const COLUMS: usize, const ROWS: usize> Matrix<COLUMS, ROWS> {
    pub fn new() -> Self {
        Self::identity()
    }

    pub fn identity() -> Self {
        let mut colums = [Vector::<ROWS>::new(); COLUMS];
        for i in 0..COLUMS {
            colums[i][i] = 1.;
        }

        Self { colums }
    }
}

impl std::ops::Mul<Vector<4>> for Matrix<3, 4> {
    type Output = Vector<3>;

    fn mul(self, rhs: Vector<4>) -> Self::Output {
        let x = dot(&self.colums[0], &rhs);
        let y = dot(&self.colums[1], &rhs);
        let z = dot(&self.colums[2], &rhs);

        Vec3::from_values(&[x, y, z])
    }
}

impl std::ops::Mul<&Vector<4>> for Matrix<3, 4> {
    type Output = Vector<3>;

    fn mul(self, rhs: &Vector<4>) -> Self::Output {
        let x = dot(&self.colums[0], &rhs);
        let y = dot(&self.colums[1], &rhs);
        let z = dot(&self.colums[2], &rhs);

        Vec3::from_values(&[x, y, z])
    }
}
