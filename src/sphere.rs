use crate::geom::{dot, Point3, Ray};
use crate::material::Material;
use crate::object::Hit;
use glam::Mat4;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Arc<Material>,
    pub transform: Mat4,
    pub inv_transform: Mat4,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<Material>, transform: Mat4) -> Self {
        let inv_transform = transform.inverse();
        Self {
            center,
            radius,
            material,
            transform,
            inv_transform,
        }
    }

    pub(crate) fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let r = r.transform(self.inv_transform);
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            };
        }
        let p = r.at(root);
        Some(Hit::new(
            self.transform.transform_point3(p),
            root,
            self.transform
                .inverse()
                .transpose()
                .transform_vector3((p - self.center) / self.radius),
            self.material.clone(),
        ))
    }
}
