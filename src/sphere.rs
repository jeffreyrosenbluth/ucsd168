use crate::geom::{dot, Point3, Ray};
use crate::object::HitRec;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub(crate) fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRec> {
        let oc = r.origin - self.center;
        let a = r.direction.length2();
        let half_b = dot(oc, r.direction);
        let c = oc.length2() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            };
        }
        let p = r.at(root);
        Some(HitRec::new(p, root))
    }
}
