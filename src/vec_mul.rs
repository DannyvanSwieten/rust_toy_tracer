use super::vec::*;

impl<const SIZE: usize> std::ops::Mul<&Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn mul(self, b: &Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] * b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Mul<Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn mul(self, b: Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] * b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Mul<f32> for Vector<SIZE> {
    type Output = Self;
    fn mul(self, b: f32) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] * b
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Mul<Vector<SIZE>> for f32 {
    type Output = Vector<SIZE>;
    fn mul(self, b: Vector<SIZE>) -> Self::Output {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self * b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Mul<f32> for &Vector<SIZE> {
    type Output = Vector<SIZE>;
    fn mul(self, b: f32) -> Vector<SIZE> {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] * b
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Mul<&Vector<SIZE>> for f32 {
    type Output = Vector<SIZE>;
    fn mul(self, b: &Vector<SIZE>) -> Self::Output {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self * b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::MulAssign<&Vector<SIZE>> for Vector<SIZE> {
    fn mul_assign(&mut self, b: &Vector<SIZE>) {
        for i in 0..SIZE {
            self.data[i] *= b.data[i]
        }
    }
}

impl<const SIZE: usize> std::ops::MulAssign<Vector<SIZE>> for Vector<SIZE> {
    fn mul_assign(&mut self, b: Vector<SIZE>) {
        for i in 0..SIZE {
            self.data[i] *= b.data[i]
        }
    }
}

impl<const SIZE: usize> std::ops::MulAssign<f32> for Vector<SIZE> {
    fn mul_assign(&mut self, b: f32) {
        for i in 0..SIZE {
            self.data[i] *= b
        }
    }
}
