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
        for i in 0..COLUMS.min(ROWS) {
            colums[i][i] = 1.;
        }

        Self { colums }
    }

    pub fn transposed(&self) -> Matrix<ROWS, COLUMS> {
        let mut result = Matrix::<ROWS, COLUMS>::new();
        for i in 0..ROWS {
            for j in 0..COLUMS {
                result.colums[i][j] = self.colums[j][i];
            }
        }
        result
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

impl std::ops::Mul<Vector<3>> for Matrix<4, 3> {
    type Output = Vector<4>;

    fn mul(self, rhs: Vector<3>) -> Self::Output {
        let x = dot(&self.colums[0], &rhs);
        let y = dot(&self.colums[1], &rhs);
        let z = dot(&self.colums[2], &rhs);
        let w = dot(&self.colums[3], &rhs);

        Vec4::from_values(&[x, y, z, w])
    }
}
