use crate::geom::{Point3, Ray};
use crate::scene::Material;
use crate::sphere::Sphere;
use crate::triangle::Triangle;

#[derive(Debug, Clone, Copy)]
pub struct HitRec {
    pub point: Point3,
    pub t: f32,
}

impl HitRec {
    pub fn new(point: Point3, t: f32) -> Self {
        Self { point, t }
    }
}

#[derive(Debug)]
pub enum Shape {
    Sphere(Sphere, Material),
    Triangle(Triangle, Material),
}

impl Shape {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRec> {
        match self {
            Shape::Sphere(s, _m) => s.hit(ray, t_min, t_max),
            Shape::Triangle(t, _m) => t.hit(ray, t_min, t_max),
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
