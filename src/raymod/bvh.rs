use crate::raymod::*;

use std::cmp::Ordering;
use std::f64;


pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vec3::new(
        f64::min(box0.min.x, box1.min.x),
        f64::min(box0.min.y, box1.min.y),
        f64::min(box0.min.z, box1.min.z),
    );
    let max = Vec3::new(
        f64::max(box0.max.x, box1.max.x),
        f64::max(box0.max.y, box1.max.y),
        f64::max(box0.max.z, box1.max.z),
    );
    AABB { min, max }
}

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

#[allow(unused)]
impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.d[a];
            let t0 = (self.min[a] - ray.o[a]) * inv_d;
            let t1 = (self.max[a] - ray.o[a]) * inv_d;
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Shape>),
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut shape: Vec<Box<dyn Shape>>) -> Self {
        fn box_compare(axis: usize) -> impl FnMut(&Box<dyn Shape>, &Box<dyn Shape>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box();
                let b_bbox = b.bounding_box();
                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min[axis] + a.max[axis];
                    let bc = b.min[axis] + b.max[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
        }

        let axis_random = random();
        let axis: usize;
        if axis_random < 0.33 {
            axis = 0
        } else if axis_random < 0.66 {
            axis = 1
        } else {
            axis = 2
        };

        shape.sort_unstable_by(box_compare(axis));
        let len = shape.len();
        match len {
            0 => panic!["no elements in scene"],
            1 => {
                let leaf = shape.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box() {
                    BVH {
                        tree: BVHNode::Leaf(leaf),
                        bbox,
                    }
                } else {
                    panic!["no bounding box in bvh node"]
                }
            }
            _ => {
                let right = BVH::new(shape.drain(len / 2..).collect());
                let left = BVH::new(shape);
                let bbox = surrounding_box(&left.bbox, &right.bbox);
                BVH {
                    tree: BVHNode::Branch {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                    bbox,
                }
            }
        }
    }
}

impl Shape for BVH {
    fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitInfo> {
        if !self.bbox.hit(&ray, t_min, t_max) {
            return None;
        }
        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(&ray, t_min, t_max),
            BVHNode::Branch { left, right } => {
                let left = left.hit(&ray, t_min, t_max);
                if let Some(l) = &left {
                    t_max = l.t
                };
                let right = right.hit(&ray, t_min, t_max);
                if right.is_some() { right } else { left }
            }
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}


