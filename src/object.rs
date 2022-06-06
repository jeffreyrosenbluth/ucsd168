use crate::aabb::{surrounding_box, Aabb};
use crate::geom::{Point3, Ray, Vec3};
use crate::material::Material;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use std::ops::Index;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Hit {
    pub point: Point3,
    pub t: f32,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

impl Hit {
    pub fn new(point: Point3, t: f32, normal: Vec3, material: Arc<Material>) -> Self {
        Self {
            point,
            t,
            material,
            normal,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Shape {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match self {
            Shape::Sphere(s) => s.hit(ray, t_min, t_max),
            Shape::Triangle(t) => t.hit(ray, t_min, t_max),
        }
    }

    pub fn bounding_box(&self) -> Aabb {
        match self {
            Shape::Sphere(s) => s.bounding_box,
            Shape::Triangle(t) => t.bounding_box,
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::Sphere(Sphere::default())
    }
}

#[derive(Debug, Clone)]
pub struct Objects(pub Vec<Shape>);

impl Objects {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut rec = None;
        let mut closest_so_far = t_max;
        for object in &self.0 {
            if let Some(new_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = new_rec.t;
                rec = Some(new_rec);
            }
        }
        rec
    }

    pub fn bounding_box(&self) -> Aabb {
        let a = self.0[0].bounding_box();
        self.0.iter().fold(a, |acc, o| {
            let b = o.bounding_box();
            surrounding_box(acc, b)
        })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Objects {
    type Output = Shape;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Default for Objects {
    fn default() -> Self {
        Self(Vec::new())
    }
}
