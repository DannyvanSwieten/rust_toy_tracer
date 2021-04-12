use super::vec::*;

// ADD
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

impl<const SIZE: usize> std::ops::Add<Vector<SIZE>> for Vector<SIZE> {
    type Output = Self;
    fn add(self, b: Vector<SIZE>) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] + b.data[i]
        }
        result
    }
}

impl<const SIZE: usize> std::ops::Add<f32> for Vector<SIZE> {
    type Output = Self;
    fn add(self, b: f32) -> Self {
        let mut result = Vector::<SIZE>::new();
        for i in 0..SIZE {
            result.data[i] = self.data[i] + b
        }
        result
    }
}
impl<const SIZE: usize> std::ops::AddAssign<&Vector<SIZE>> for Vector<SIZE> {
    fn add_assign(&mut self, b: &Vector<SIZE>) {
        for i in 0..SIZE {
            self.data[i] += b.data[i]
        }
    }
}
impl<const SIZE: usize> std::ops::AddAssign<f32> for Vector<SIZE> {
    fn add_assign(&mut self, b: f32) {
        for i in 0..SIZE {
            self.data[i] += b
        }
    }
}
