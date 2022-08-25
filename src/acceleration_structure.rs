use slotmap::DefaultKey;
use slotmap::SlotMap;

use super::bounding_box::*;
use super::hittable::*;
use super::rand::*;
use super::ray::*;
use super::scene::*;
use super::types::{Position, Transform};
use super::vec::*;

trait Node {
    fn hit_test(&self, ray: &Ray, t_min: f32, t_max: f32, result: Vec<u32>) -> Vec<u32>;
}

struct Branch {
    left: Box<dyn Node>,
    right: Box<dyn Node>,
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
    fn new(instance: (u32, BoundingBox)) -> Box<dyn Node> {
        Box::new(Self {
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
    fn new(instances: &mut Vec<(u32, BoundingBox)>) -> Box<dyn Node> {
        let mut bounding_box = BoundingBox::new(
            Position::from_values([0., 0., 0.]),
            Position::from_values([0., 0., 0.]),
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
        Box::new(Self {
            left,
            right,
            bounding_box,
        })
    }

    fn from_slice(slice: &mut [(u32, BoundingBox)]) -> Box<dyn Node> {
        let mut bounding_box = BoundingBox::new(
            Position::from_values([0., 0., 0.]),
            Position::from_values([0., 0., 0.]),
        );
        for (_, bb) in slice.iter() {
            bounding_box = BoundingBox::surrounding_box(&bounding_box, &bb);
        }

        if slice.len() == 1 {
            return Box::new(Leaf {
                id: slice[0].0,
                bounding_box: slice[0].1,
            });
        }

        let mid = slice.len() / 2;
        let (left_instances, right_instances) = slice.split_at_mut(mid);
        let left = Self::from_slice(left_instances);
        let right = Self::from_slice(right_instances);
        Box::new(Self {
            left,
            right,
            bounding_box,
        })
    }
}
pub struct TopLevelAccelerationStructure {
    instances: Vec<Instance>,
    bounding_box: BoundingBox,
    root_node: Box<dyn Node>,
}

unsafe impl Send for TopLevelAccelerationStructure {}
unsafe impl Sync for TopLevelAccelerationStructure {}

impl TopLevelAccelerationStructure {
    pub fn new(
        hittables: &SlotMap<DefaultKey, Box<dyn Hittable>>,
        instances: &Vec<Instance>,
    ) -> Self {
        let geometry = hittables.clone();
        let geometry_instances = instances.clone();
        let mut id_and_bb = Vec::new();

        // collect bounding boxes and id's
        for instance in instances.iter() {
            id_and_bb.push((
                instance.instance_id,
                geometry[instance.geometry_index]
                    .bounding_box()
                    .unwrap()
                    .transformed(&instance.transform),
            ))
        }

        // sort them on x-axis (arbitrary axis)
        id_and_bb.sort_by(|(_, bb), (_, bb2)| bb.min().x().partial_cmp(&bb2.max().x()).unwrap());
        let mut bounding_box = BoundingBox::new(
            Position::from_values([0., 0., 0.]),
            Position::from_values([0., 0., 0.]),
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
            //hittables: geometry,
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

    pub fn geometry(&self, index: usize) {
        //&self.hittables[index]
    }

    pub fn instance(&self, id: usize) -> &Instance {
        &self.instances[id]
    }
}

#[derive(Default, Clone)]
struct BVHFlatNode {
    parent_idx: u32,
    left_child_idx: u32,
    right_child_idx: u32,
    primitive_idx: u32,
}

impl BVHFlatNode {
    fn is_leaf(&self) -> bool {
        self.primitive_idx != 0xFFFFFFFF
    }
}

struct MortonCode {
    code: u32,
    primitive_id: u32,
}

impl MortonCode {
    fn expand_bits(mut v: u32) -> u32 {
        assert_eq!(v & 0x03000000, 0);
        v = (v | (v << 16)) & 0x030000FF;
        v = (v | (v << 8)) & 0x0300F00F;
        v = (v | (v << 4)) & 0x030C30C3;
        v = (v | (v << 2)) & 0x09249249;
        v
    }

    pub fn new(position: &Position, primitive_id: u32) -> Self {
        let c = max(&(position * 1024.0), &Position::new());
        let x = c[0].min(1023.0) as u32;
        let y = c[1].min(1023.0) as u32;
        let z = c[2].min(1023.0) as u32;
        let x = Self::expand_bits(x);
        let y = Self::expand_bits(y);
        let z = Self::expand_bits(z);

        let code = x | (y << 1) | (z << 2);
        Self {
            code: code as u32,
            primitive_id,
        }
    }
}

pub struct BottomLevelAccelerationStructure {
    total_bb: BoundingBox,
    bbs: Vec<BoundingBox>,
    nodes: Vec<BVHFlatNode>,
}

impl BottomLevelAccelerationStructure {
    pub fn bounding_box(&self) -> BoundingBox {
        self.total_bb
    }

    fn hit_internal(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        node: &BVHFlatNode,
        mut results: Vec<u32>,
    ) -> Vec<u32> {
        let left_bb = &self.bbs[node.left_child_idx as usize];
        let right_bb = &self.bbs[node.right_child_idx as usize];
        let left_hit = left_bb.transformed(object_to_world).hit(ray, t_min, t_max);
        if left_hit {
            if self.nodes[node.left_child_idx as usize].is_leaf() {
                results.push(self.nodes[node.left_child_idx as usize].primitive_idx)
            } else {
                results = self.hit_internal(
                    object_to_world,
                    ray,
                    t_min,
                    t_max,
                    &self.nodes[node.left_child_idx as usize],
                    results,
                )
            }
        }
        let right_hit = right_bb.transformed(object_to_world).hit(ray, t_min, t_max);
        if right_hit {
            if self.nodes[node.right_child_idx as usize].is_leaf() {
                results.push(self.nodes[node.right_child_idx as usize].primitive_idx)
            } else {
                results = self.hit_internal(
                    object_to_world,
                    ray,
                    t_min,
                    t_max,
                    &self.nodes[node.right_child_idx as usize],
                    results,
                )
            }
        }
        results
    }

    pub fn hit_test(
        &self,
        object_to_world: &Transform,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Vec<u32> {
        let mut results = Vec::new();
        if self.bbs[0]
            .transformed(object_to_world)
            .hit(ray, t_min, t_max)
        {
            results =
                self.hit_internal(object_to_world, ray, t_min, t_max, &self.nodes[0], results);
        }

        results
    }

    fn find_range(codes: &Vec<MortonCode>, mut idx: usize) -> (usize, usize) {
        if idx == 0 {
            return (0, codes.len() - 1);
        }

        let self_code = codes[idx].code;
        let l_delta = (self_code ^ codes[idx - 1].code).leading_zeros() as i32;
        let r_delta = (self_code ^ codes[idx + 1].code).leading_zeros() as i32;
        let d = if r_delta > l_delta { 1 } else { -1 };
        let delta_min = l_delta.min(r_delta) as i32;
        let mut l_max = 2;
        let mut delta = -1;
        let mut i_tmp = idx as i32 + d * l_max;
        if 0 <= i_tmp && i_tmp < codes.len() as i32 {
            delta = (self_code ^ codes[i_tmp as usize].code).leading_zeros() as i32;
        }

        while delta > delta_min {
            l_max <<= 1;
            i_tmp = idx as i32 + d * l_max;
            delta = -1;
            if 0 <= i_tmp && i_tmp < codes.len() as i32 {
                delta = (self_code ^ codes[i_tmp as usize].code).leading_zeros() as i32;
            }
        }

        let mut l = 0;
        let mut t = l_max >> 1;
        while t > 0 {
            i_tmp = idx as i32 + (l + t) * d;
            delta = -1;
            if 0 <= i_tmp && i_tmp < codes.len() as i32 {
                delta = (self_code ^ codes[i_tmp as usize].code).leading_zeros() as i32;
            }
            if delta > delta_min {
                l += t;
            }
            t >>= 1;
        }
        let mut jdx = idx as i32 + l * d;
        if d < 0 {
            let tmp = idx as i32;
            idx = jdx as usize;
            jdx = tmp;
        }

        (idx, jdx as usize)
    }

    fn find_split(codes: &Vec<MortonCode>, first: usize, last: usize) -> usize {
        let c1 = codes[first].code;
        let c2 = codes[last].code;
        if c1 == c2 {
            return ((first + last) >> 1) as usize;
        }

        let delta_node = (c1 ^ c2).leading_zeros();
        let mut split = first;
        let mut stride = last - first;
        while stride > 1 {
            stride = (stride + 1) >> 1;
            let middle = split + stride;
            if middle < last {
                let split_code = codes[middle].code;
                let delta = (c1 ^ split_code).leading_zeros();
                if delta > delta_node {
                    split = middle
                }
            }
        }

        split
    }

    pub fn new(vertices: &Vec<Position>, indices: Option<&Vec<u32>>) -> Self {
        let leaf_count = indices.unwrap().len() / 3;
        let branch_count = leaf_count - 1;
        let total_node_count = leaf_count + branch_count;
        let mut primitive_bbs = vec![BoundingBox::default(); total_node_count];
        let mut total_bb = BoundingBox::new(
            Position::from_values([f32::MAX, f32::MAX, f32::MAX]),
            Position::from_values([f32::MIN, f32::MIN, f32::MIN]),
        );
        if let Some(indices) = &indices {
            for i in (0..indices.len()).step_by(3) {
                let idx0 = indices[i] as usize;
                let idx1 = indices[i + 1] as usize;
                let idx2 = indices[i + 2] as usize;
                let min = min(&vertices[idx0], &min(&vertices[idx1], &vertices[idx2]));
                let max = max(&vertices[idx0], &max(&vertices[idx1], &vertices[idx2]));
                let triangle_bb = BoundingBox::new(min, max);
                total_bb = BoundingBox::surrounding_box(&triangle_bb, &total_bb);
                let leaf_idx = (i / 3) + branch_count;
                primitive_bbs[leaf_idx] = triangle_bb;
            }
        } else {
            for i in (0..vertices.len()).step_by(3) {
                let min = min(&vertices[i], &min(&vertices[i + 1], &vertices[i + 2]));
                let max = max(&vertices[i], &max(&vertices[i + 1], &vertices[i + 2]));
                let triangle_bb = BoundingBox::new(min, max);
                total_bb = BoundingBox::surrounding_box(&triangle_bb, &total_bb);
                primitive_bbs.push(triangle_bb);
            }
        }

        let mut morton_codes: Vec<MortonCode> = primitive_bbs
            .iter()
            .skip(branch_count)
            .enumerate()
            .map(|(index, bb)| {
                let normalized_mid_point = total_bb.relative_position(&bb.center());
                MortonCode::new(&normalized_mid_point, index as u32)
            })
            .collect();

        morton_codes.sort_by_key(|code| code.code);
        let mut b: Vec<BoundingBox> = morton_codes
            .iter()
            .map(|code| primitive_bbs[code.primitive_id as usize + branch_count])
            .collect();

        for _ in 0..branch_count {
            b.insert(0, BoundingBox::default())
        }

        primitive_bbs = b;
        let mut nodes = vec![BVHFlatNode::default(); total_node_count];
        nodes[0].parent_idx = 0xFFFFFFFF;
        for i in 0..branch_count {
            nodes[i].primitive_idx = 0xFFFFFFFF;
        }

        for i in branch_count..total_node_count {
            nodes[i].primitive_idx = morton_codes[i - branch_count].primitive_id;
        }

        for i in 0..branch_count {
            let (first, last) = Self::find_range(&morton_codes, i);
            let split = Self::find_split(&morton_codes, first, last);

            nodes[i].primitive_idx = 0xFFFFFFFF;

            nodes[i].left_child_idx = split as u32;
            if first.min(last) == split {
                nodes[i].left_child_idx += branch_count as u32;
            }
            nodes[i].right_child_idx = (split + 1) as u32;
            if first.max(last) == split + 1 {
                nodes[i].right_child_idx += branch_count as u32
            }

            let left_idx = nodes[i].left_child_idx as usize;
            nodes[left_idx].parent_idx = i as u32;

            let right_idx = nodes[i].right_child_idx as usize;
            nodes[right_idx].parent_idx = i as u32;
        }

        //primitive_bbs[0] = total_bb;
        let mut flags = vec![false; branch_count];

        for i in branch_count..nodes.len() {
            let mut parent = nodes[i].parent_idx as usize;
            while parent != 0xFFFFFFFF {
                if flags[parent] {
                    let lidx = nodes[parent].left_child_idx as usize;
                    let ridx = nodes[parent].right_child_idx as usize;
                    let bb_l = &primitive_bbs[lidx];
                    let bb_r = &primitive_bbs[ridx];
                    primitive_bbs[parent] = BoundingBox::surrounding_box(bb_l, bb_r);
                    parent = nodes[parent].parent_idx as usize;
                } else {
                    flags[parent] = true;
                }
            }
        }

        Self {
            total_bb,
            bbs: primitive_bbs,
            nodes,
        }
    }
}
