use crate::geom::{Point3, Ray};
use crate::scene::Material;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct HitRec {
    pub point: Point3,
    pub t: f32,
    pub material: Arc<Material>,
}

impl HitRec {
    pub fn new(point: Point3, t: f32, material: Arc<Material>) -> Self {
        Self { point, t, material }
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Shape {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRec> {
        match self {
            Shape::Sphere(s) => s.hit(ray, t_min, t_max),
            Shape::Triangle(t) => t.hit(ray, t_min, t_max),
        }
    }
}

#[derive(Debug)]
pub struct Objects(pub Vec<Shape>);

impl Objects {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRec> {
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
}
