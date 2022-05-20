use crate::geom::{cross, dot, Point3, Ray};
use crate::object::Hit;
use crate::material::Material;
use glam::Mat4;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertex1: Point3,
    pub vertex2: Point3,
    pub vertex3: Point3,
    pub material: Arc<Material>,
    pub transform: Mat4,
    pub inv_transform: Mat4,
}

impl Triangle {
    pub fn new(
        vertex1: Point3,
        vertex2: Point3,
        vertex3: Point3,
        material: Arc<Material>,
        transform: Mat4,
    ) -> Self {
        let inv_transform = transform.inverse();
        Triangle {
            vertex1,
            vertex2,
            vertex3,
            material,
            transform,
            inv_transform,
        }
    }

    pub(crate) fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let ray = ray.transform(self.inv_transform);
        const EPS1: f32 = 1e-7;
        const EPS2: f32 = 1e-10;

        let e1 = self.vertex2 - self.vertex1;
        let e2 = self.vertex3 - self.vertex1;
        let q = cross(ray.direction, e2);
        let a = dot(e1, q);
        let s = ray.origin - self.vertex1;
        let r = cross(s, e1);
        let w2 = dot(s, q) / a;
        let w3 = dot(ray.direction, r) / a;
        let w1 = 1.0 - w2 - w3;
        let t = dot(e2, r) / a;

        if a <= EPS1 || w1 < -EPS2 || w2 < -EPS2 || w3 < -EPS2 || t < t_min || t > t_max {
            None
        } else {
            let p = w1 * self.vertex1 + w2 * self.vertex2 + w3 * self.vertex3;
            Some(Hit::new(
                self.transform.transform_point3(p),
                t,
                self.material.clone(),
            ))
        }
    }
}
