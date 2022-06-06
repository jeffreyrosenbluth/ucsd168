use crate::aabb::*;
use crate::geom::{Ray, Vec3};
use crate::object::{Hit, Objects};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub enum Node {
    Branch {
        left: Box<Node>,
        right: Box<Node>,
        bbox: Aabb,
    },
    Leaf {
        shape_index: usize,
        bbox: Aabb,
    },
    Empty,
}

impl Node {
    pub fn bbox(&self) -> Aabb {
        match self {
            Node::Leaf {
                shape_index: _,
                bbox,
            } => *bbox,
            Node::Branch {
                left: _,
                right: _,
                bbox,
            } => *bbox,
            Node::Empty => Aabb::new(Vec3::ZERO, Vec3::ZERO),
        }
    }

    pub fn new(objects: &Objects, indices: Vec<usize>, axis: usize) -> Self {
        let object_span = indices.len();
        let node = match object_span {
            0 => Node::Empty,
            1 => {
                let i = indices[0];
                let bbox = objects[i].bounding_box();
                Node::Leaf {
                    shape_index: i,
                    bbox,
                }
            }

            2 => {
                let i0 = indices[0];
                let i1 = indices[1];
                let first_bbox = objects[i0].bounding_box();
                let second_bbox = objects[i1].bounding_box();
                let (f, s) = match first_bbox.compare(&second_bbox, axis) {
                    Ordering::Less => (i0, i1),
                    _ => (i1, i0),
                };
                Node::Branch {
                    left: Box::new(Node::Leaf {
                        shape_index: f,
                        bbox: first_bbox,
                    }),
                    right: Box::new(Node::Leaf {
                        shape_index: s,
                        bbox: second_bbox,
                    }),
                    bbox: surrounding_box(first_bbox, second_bbox),
                }
            }
            _ => {
                let mut mean = 0.0;
                let n = indices.len() as f32;
                for k in &indices {
                    mean += objects[*k].bounding_box().box_min[axis];
                }
                mean /= n;
                let mut l = Vec::new();
                let mut r = Vec::new();
                for i in &indices {
                    if objects[*i].bounding_box().box_min[axis] == mean {
                        if l.len() < r.len() {
                            l.push(*i);
                        } else {
                            r.push(*i);
                        };
                    } else if objects[*i].bounding_box().box_min[axis] < mean {
                        l.push(*i)
                    } else {
                        r.push(*i)
                    }
                }
                let left = Self::new(objects, l, (axis + 1) % 3);
                let right = Self::new(objects, r, (axis + 1) % 3);
                let left_bbox = left.bbox();
                let right_bbox = right.bbox();

                Node::Branch {
                    left: Box::new(left),
                    right: Box::new(right),
                    bbox: surrounding_box(left_bbox, right_bbox),
                }
            }
        };
        node
    }

    pub fn hit(&self, objects: &Objects, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        if !self.bbox().hit(ray, t_min, t_max) {
            return None;
        };
        match self {
            Node::Branch {
                left,
                right,
                bbox: _,
            } => {
                let left_rec = left.hit(objects, ray, t_min, t_max);
                let t = if let Some(rec) = &left_rec {
                    rec.t
                } else {
                    t_max
                };
                let right_rec = right.hit(objects, ray, t_min, t);
                right_rec.or(left_rec)
            }
            Node::Leaf {
                shape_index,
                bbox: _,
            } => objects[*shape_index].hit(ray, t_min, t_max),
            Node::Empty => None,
        }
    }
}
