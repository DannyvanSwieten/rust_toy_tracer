use super::ray::*;
use super::types::*;
use super::vec::*;

#[derive(Copy, Clone)]
pub struct BoundingBox {
    min: Position,
    max: Position,
}

impl BoundingBox {
    pub fn new(min: &Position, max: &Position) -> Self {
        Self {
            min: *min,
            max: *max,
        }
    }

    pub fn transformed(&self, transform: &Transform) -> Self {
        let min = *transform * &Vec4::from(self.min);
        let max = *transform * &Vec4::from(self.max);

        Self::new(&min, &max)
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

        BoundingBox::new(&small, &big)
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
