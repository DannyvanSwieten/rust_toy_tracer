use super::vec::*;

// ADD
impl<const SIZE: usize> std::ops::Div<&Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn div(self, b: &Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] / b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Div<Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn div(self, b: Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] / b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Div<f32> for Vector<SIZE> {
    type Output = Self;
    fn div(self, b: f32) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] / b
        }
        result
    }
}
impl<const SIZE: usize> std::ops::DivAssign<&Vector<SIZE>> for Vector<SIZE> {
    fn div_assign(&mut self, b: &Vector<SIZE>) {
        for i in 0..SIZE {
            self.data[i] /= b.data[i]
        }
    }
}
impl<const SIZE: usize> std::ops::DivAssign<f32> for Vector<SIZE> {
    fn div_assign(&mut self, b: f32) {
        for i in 0..SIZE {
            self.data[i] /= b
        }
    }
}
