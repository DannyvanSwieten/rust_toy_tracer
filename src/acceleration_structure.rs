use super::bounding_box::*;
use super::hittable::*;
use super::rand::*;
use super::ray::*;
use super::scene::*;
use super::types::Position;
use super::vec::*;
use std::sync::Arc;

trait Node {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32, result: Vec<u32>) -> Vec<u32>;
}

struct Branch {
    left: Arc<dyn Node + Send + Sync>,
    right: Arc<dyn Node + Send + Sync>,
    bounding_box: BoundingBox,
}

impl Node for Branch {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32, result: Vec<u32>) -> Vec<u32> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            result
        } else {
            let ids = self.left.hit_test(ray, t_min, t_max, result);
            let r = self.right.hit_test(ray, t_min, t_max, ids);
            r
        }
    }
}

struct Leaf {
    id: u32,
    bounding_box: BoundingBox,
}

impl Leaf {
    fn new(instance: (u32, BoundingBox)) -> Arc<dyn Node + Send + Sync> {
        Arc::new(Self {
            id: instance.0,
            bounding_box: instance.1,
        })
    }
}

impl Node for Leaf {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32, mut result: Vec<u32>) -> Vec<u32> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            result.push(self.id);
            result
        } else {
            result
        }
    }
}

impl Branch {
    fn new(instances: &mut Vec<(u32, BoundingBox)>) -> Arc<dyn Node + Send + Sync> {
        let mut bounding_box = BoundingBox::new(
            &Position::from_values(&[0., 0., 0.]),
            &Position::from_values(&[0., 0., 0.]),
        );
        for (_, bb) in instances.iter() {
            bounding_box = BoundingBox::surrounding_box(&bounding_box, &bb);
        }

        let i = int_range(0, 3) as usize;
        instances.sort_by(|(_, bb), (_, bb2)| bb.min()[i].partial_cmp(&bb2.max()[i]).unwrap());

        let mid = instances.len() / 2;
        let (left_instances, right_instances) = instances.split_at_mut(mid);
        let left = Self::from_slice(left_instances);
        let right = Self::from_slice(right_instances);
        Arc::new(Self {
            left,
            right,
            bounding_box,
        })
    }

    fn from_slice(slice: &mut [(u32, BoundingBox)]) -> Arc<dyn Node + Send + Sync> {
        let mut bounding_box = BoundingBox::new(
            &Position::from_values(&[0., 0., 0.]),
            &Position::from_values(&[0., 0., 0.]),
        );
        for (_, bb) in slice.iter() {
            bounding_box = BoundingBox::surrounding_box(&bounding_box, &bb);
        }

        if slice.len() == 1 {
            return Arc::new(Leaf {
                id: slice[0].0,
                bounding_box: slice[0].1,
            });
        }

        let mid = slice.len() / 2;
        let (left_instances, right_instances) = slice.split_at_mut(mid);
        let left = Self::from_slice(left_instances);
        let right = Self::from_slice(right_instances);
        Arc::new(Self {
            left,
            right,
            bounding_box,
        })
    }
}

pub struct AccelerationStructure {
    hittables: Vec<Arc<dyn Hittable + Send + Sync>>,
    instances: Vec<Instance>,
    bounding_box: BoundingBox,
    root_node: Arc<dyn Node + Send + Sync>,
}

impl AccelerationStructure {
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
                instance.instance_id,
                geometry[instance.geometry_index as usize]
                    .bounding_box()
                    .unwrap()
                    .transformed(&instance.transform),
            ))
        }

        // sort them on x-axis (arbitrary axis)
        id_and_bb.sort_by(|(_, bb), (_, bb2)| bb.min().x().partial_cmp(&bb2.max().x()).unwrap());
        let mut bounding_box = BoundingBox::new(
            &Position::from_values(&[0., 0., 0.]),
            &Position::from_values(&[0., 0., 0.]),
        );

        // calculate the bounding box of the entire structure.
        for (_, bb) in id_and_bb.iter() {
            bounding_box = BoundingBox::surrounding_box(&bounding_box, &bb);
        }

        let root_node = if id_and_bb.len() > 1 {
            Branch::new(&mut id_and_bb)
        } else {
            Leaf::new(id_and_bb[0])
        };

        Self {
            hittables: geometry,
            instances: geometry_instances,
            bounding_box,
            root_node,
        }
    }

    pub fn intersect_instance(&self, ray: &Ray, t_min: f32, t_max: f32) -> Vec<u32> {
        let results = Vec::new();
        if self.bounding_box.hit(ray, t_min, t_max) {
            self.root_node.hit_test(ray, t_min, t_max, results)
        } else {
            results
        }
    }

    pub fn geometry(&self, index: usize) -> &Arc<dyn Hittable + Send + Sync> {
        &self.hittables[index]
    }

    pub fn instance(&self, id: usize) -> &Instance {
        &self.instances[id]
    }
}
