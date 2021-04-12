use super::vec::*;

// ADD
impl<const SIZE: usize> std::ops::Sub<&Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn sub(self, b: &Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] - b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Sub<Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn sub(self, b: Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] - b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Sub<f32> for Vector<SIZE> {
    type Output = Self;
    fn sub(self, b: f32) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] - b
        }
        result
    }
}
impl<const SIZE: usize> std::ops::SubAssign<&Vector<SIZE>> for Vector<SIZE> {
    fn sub_assign(&mut self, b: &Vector<SIZE>) {
        for i in 0..SIZE {
            self.data[i] -= b.data[i]
        }
    }
}
impl<const SIZE: usize> std::ops::SubAssign<f32> for Vector<SIZE> {
    fn sub_assign(&mut self, b: f32) {
        for i in 0..SIZE {
            self.data[i] -= b
        }
    }
}

impl<const SIZE: usize> std::ops::Neg for Vector<SIZE> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for i in 0..SIZE {
            self.data[i] = -self.data[i]
        }

        self
    }
}
