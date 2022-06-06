use crate::geom::*;
use crate::material::Material;
use crate::object::Hit;
use glam::Vec3;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub box_min: Vec3,
    pub box_max: Vec3,
}

impl Aabb {
    pub fn new(box_min: Vec3, box_max: Vec3) -> Self {
        Self { box_min, box_max }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let mut t = t_max;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.box_min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.box_max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0)
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max <= t_min {
                return false;
            }
            if t_min < t {
                t = t_min
            };
        }
        true
    }

    pub fn compare(&self, other: &Self, axis: usize) -> Ordering {
        let x = self.box_min[axis];
        let y = other.box_min[axis];
        if x < y {
            Ordering::Less
        } else if x > y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = point3(
        box0.box_min.x.min(box1.box_min.x),
        box0.box_min.y.min(box1.box_min.y),
        box0.box_min.z.min(box1.box_min.z),
    );

    let large = point3(
        box0.box_max.x.max(box1.box_max.x),
        box0.box_max.y.max(box1.box_max.y),
        box0.box_max.z.max(box1.box_max.z),
    );

    Aabb {
        box_min: small,
        box_max: large,
    }
}
