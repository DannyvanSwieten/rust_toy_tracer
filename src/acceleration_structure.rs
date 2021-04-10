use super::bounding_box::*;
use super::hittable::*;
use super::intersection::*;
use super::rand_float::rand_range;
use super::ray::*;
use super::scene::*;
use super::types::Position;
use std::sync::Arc;

struct AccelerationStructureNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bounding_box: BoundingBox,
}

pub struct AccelerationStructV2 {
    hittables: Vec<Arc<dyn Hittable + Send + Sync>>,
    instances: Vec<Instance>,
    bounding_box: BoundingBox,
    //root_node: AccelerationStructureNode,
}

impl AccelerationStructV2 {
    pub fn new(
        hittables: &Vec<Arc<dyn Hittable + Send + Sync>>,
        instances: &Vec<Instance>,
    ) -> Self {
        let geometry = hittables.clone();
        let geometry_instances = instances.clone();
        let mut id_and_bb = Vec::new();

        // collect bounding boxes and id's
        for instance in instances.iter() {
            id_and_bb.push((
                instance.object_id,
                geometry[instance.object_id as usize]
                    .bounding_box()
                    .unwrap()
                    .transformed(&instance.transform),
            ))
        }

        // sort them on x-axis (arbitrary axis)
        id_and_bb.sort_by(|a, b| a.1.min().x.partial_cmp(&b.1.max().x).unwrap());
        let mut bounding_box =
            BoundingBox::new(&Position::new(0., 0., 0.), &Position::new(0., 0., 0.));

        // calculate the bounding box of the entire structure.
        for (_, bb) in id_and_bb.iter() {
            bounding_box = BoundingBox::surrounding_box(&bounding_box, &bb);
        }

        Self {
            hittables: geometry,
            instances: geometry_instances,
            bounding_box,
        }
    }
}

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
            if let Some(left_hit) = self.left.intersect(ray, t_min, t_max) {
                if let Some(right_hit) = self.right.intersect(ray, t_min, left_hit.t) {
                    return Some(right_hit);
                } else {
                    return Some(left_hit);
                }
            } else {
                return self.right.intersect(ray, t_min, t_max);
            }
        }
    }
    fn bounding_box(&self) -> Option<BoundingBox> {
        Some(self.bounding_box)
    }
}

impl AccelerationStructure {
    pub fn from_hittables_and_instances(
        hittables: &Vec<Arc<dyn Hittable + Send + Sync>>,
        instances: &Vec<Instance>,
    ) -> Self {
        let clones = hittables.clone();
        Self::from_hittables(clones)
    }

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
        let r = rand_range(0., 3.) as u32;

        if r == 0 {
            hittables.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .x
                    .partial_cmp(&b.bounding_box().unwrap().max().x)
                    .unwrap()
            });
        } else if r == 1 {
            hittables.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .y
                    .partial_cmp(&b.bounding_box().unwrap().max().y)
                    .unwrap()
            });
        } else {
            hittables.sort_by(|a, b| {
                a.bounding_box()
                    .unwrap()
                    .min()
                    .z
                    .partial_cmp(&b.bounding_box().unwrap().max().z)
                    .unwrap()
            });
        }

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
            // Sort them on random axis
            let r = rand_range(0., 3.) as u32;

            if r == 0 {
                clones.sort_by(|a, b| {
                    a.bounding_box()
                        .unwrap()
                        .min()
                        .x
                        .partial_cmp(&b.bounding_box().unwrap().max().x)
                        .unwrap()
                });
            } else if r == 1 {
                clones.sort_by(|a, b| {
                    a.bounding_box()
                        .unwrap()
                        .min()
                        .y
                        .partial_cmp(&b.bounding_box().unwrap().max().y)
                        .unwrap()
                });
            } else {
                clones.sort_by(|a, b| {
                    a.bounding_box()
                        .unwrap()
                        .min()
                        .z
                        .partial_cmp(&b.bounding_box().unwrap().max().z)
                        .unwrap()
                });
            }

            let (left, right) = clones.split_at_mut(clones.len() / 2);
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

    pub fn intersec_bounding_box(&self, t_min: f32, t_max: f32) {}
}
