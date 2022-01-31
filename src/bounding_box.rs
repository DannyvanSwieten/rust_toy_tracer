use super::ray::*;
use super::types::*;
use super::vec::*;

#[derive(Copy, Clone, Default)]
pub struct BoundingBox {
    min: Position,
    max: Position,
}

impl BoundingBox {
    pub fn new(min: Position, max: Position) -> Self {
        let mut bb = Self { min, max };
        let dims = bb.dimensions();
        if dims.x() == 0.0 {
            bb.max[0] += 0.0001;
        }

        if dims.y() == 0.0 {
            bb.max[1] += 0.0001;
        }

        if dims.z() == 0.0 {
            bb.max[2] += 0.0001;
        }

        bb
    }

    pub fn dimensions(&self) -> Vec3 {
        let diff = self.max - self.min;
        abs(&diff)
    }

    pub fn diagonal_length(&self) -> f32 {
        let diff = self.max - self.min;
        length(&diff)
    }

    pub fn center(&self) -> Position {
        let c = self.min + self.max;
        c * 0.5
    }

    pub fn to_origin(&self) -> Self {
        let min = Position::new();
        let max = self.max - self.min;
        Self { min, max }
    }

    pub fn with_offset(self, o: &Position) -> Self {
        Self::new(self.min + o, self.max + o)
    }

    pub fn relative_position(&self, position: &Position) -> Position {
        (*position - self.min()) / self.dimensions()
    }

    pub fn transformed(&self, transform: &Transform) -> Self {
        let min = *transform * &Vec4::from(self.min);
        let max = *transform * &Vec4::from(self.max);

        Self::new(min, max)
    }

    pub fn min(&self) -> &Position {
        &self.min
    }

    pub fn max(&self) -> &Position {
        &self.max
    }

    pub fn surrounding_box(a: &BoundingBox, b: &BoundingBox) -> Self {
        let small = min(&a.min, &b.min);
        let big = max(&a.max, &b.max);

        BoundingBox::new(small, big)
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let mut t0 = (self.min[a] - ray.origin()[a]) * ray.inv_direction()[a];
            let mut t1 = (self.max[a] - ray.origin()[a]) * ray.inv_direction()[a];
            if ray.inv_direction()[a] < 0. {
                std::mem::swap(&mut t0, &mut t1)
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod vec_tests {
    use crate::types::Position;

    use super::BoundingBox;

    #[test]
    fn test_abs() {
        let bb = BoundingBox::new(
            Position::from_values(&[-5.0, -5.0, -5.0]),
            Position::from_values(&[5.0, 5.0, 5.0]),
        );

        let p = Position::from_values(&[2.5, 2.5, 2.5]);
        let relative = bb.relative_position(&p);
        assert_eq!(relative.data[0], 0.75);

        let p = Position::from_values(&[-2.5, -2.5, -2.5]);
        let relative = bb.relative_position(&p);
        assert_eq!(relative.data[0], 0.25)
    }
}
