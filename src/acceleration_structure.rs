use super::bounding_box::*;
use super::hittable::*;
use super::intersection::*;
use super::ray::*;
use super::scene::*;
use std::sync::Arc;

pub struct AccelerationStructure {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bounding_box: BoundingBox,
}

impl Hittable for AccelerationStructure {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        } else {
            let left = self.left.intersect(ray, t_min, t_max);

            if let Some(left_hit) = left {
                let right = self.right.intersect(ray, t_min, left_hit.t);
                if right.is_some() {
                    return right;
                } else {
                    return Some(left_hit);
                }
            } else {
                let right = self.right.intersect(ray, t_min, t_max);
                return right;
            }
        }

        None
    }
    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(self.bounding_box)
    }
}

impl AccelerationStructure {
    pub fn new(scene: &Scene) -> Self {
        // Clone the hittables
        let hittables = scene.hittables().clone();
        // Build acceleration structure
        Self::from_hittables(hittables)
    }

    fn from_hittables(hittables: Vec<Arc<dyn Hittable + Send + Sync>>) -> Self {
        // Find the midpoint.
        let mid = hittables.len() / 2;
        // Move to a mutable
        let mut hittables = hittables;

        // Sort them on random axis
        hittables.sort_by(|a, b| {
            a.bounding_box()
                .unwrap()
                .min()
                .x
                .partial_cmp(&b.bounding_box().unwrap().max().x)
                .unwrap()
        });

        // Split in the center
        let (left, right) = hittables.split_at_mut(mid);

        let left_node = Self::from_slice(left);
        let right_node = Self::from_slice(right);

        let a = left_node.bounding_box().unwrap();
        let b = right_node.bounding_box().unwrap();
        let c = BoundingBox::surrounding_box(&a, &b);

        Self {
            left: left_node,
            right: right_node,
            bounding_box: c,
        }
    }

    fn from_slice(
        hittables: &mut [Arc<dyn Hittable + Send + Sync>],
    ) -> Arc<dyn Hittable + Send + Sync> {
        let clones = hittables;
        if clones.len() == 1 {
            let a = clones[0].bounding_box();
            return Arc::new(Self {
                left: clones[0].clone(),
                right: clones[0].clone(),
                bounding_box: a.unwrap(),
            });
        } else if clones.len() == 2 {
            let a = clones[0].bounding_box().unwrap();
            let b = clones[1].bounding_box().unwrap();
            let c = BoundingBox::surrounding_box(&a, &b);
            return Arc::new(Self {
                left: clones[0].clone(),
                right: clones[1].clone(),
                bounding_box: c,
            });
        } else {
            clones.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .x
                    .partial_cmp(&b.bounding_box().unwrap().max().x)
                    .unwrap()
            });

            let (left, right) = clones.split_at_mut(clones.len() / 2 + 1);
            let left_node = Self::from_slice(left);
            let right_node = Self::from_slice(right);

            let a = left_node.bounding_box().unwrap();
            let b = right_node.bounding_box().unwrap();
            let c = BoundingBox::surrounding_box(&a, &b);

            return Arc::new(Self {
                left: left_node,
                right: right_node,
                bounding_box: c,
            });
        }
    }
}