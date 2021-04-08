use super::glm::builtin::*;
use super::ray::*;
use super::types::*;

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

    pub fn min(&self) -> &Position {
        &self.min
    }

    pub fn max(&self) -> &Position {
        &self.max
    }

    pub fn surrounding_box(a: &BoundingBox, b: &BoundingBox) -> Self {
        let small = min(a.min, b.min);
        let big = max(a.max, b.max);

        BoundingBox::new(&small, &big)
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1. / ray.direction()[a];
            let mut t0 = (self.min[a] - ray.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin()[a]) * inv_d;
            if inv_d < 0. {
                std::mem::swap(&mut t0, &mut t1)
            }

            let min = if t0 > t_min { t0 } else { t_min };
            let max = if t1 < t_max { t1 } else { t_max };

            if max <= min {
                return false;
            }
        }

        true
    }
}
