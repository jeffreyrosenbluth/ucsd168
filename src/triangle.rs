use crate::geom::{cross, dot, Point3, Ray};
use crate::object::HitRec;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub vertex1: Point3,
    pub vertex2: Point3,
    pub vertex3: Point3,
}

impl Triangle {
    pub fn new(vertex1: Point3, vertex2: Point3, vertex3: Point3) -> Self {
        Triangle {
            vertex1,
            vertex2,
            vertex3,
        }
    }

    pub(crate) fn hit(&self, ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRec> {
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

        if a <= EPS1 || w1 < -EPS2 || w2 < -EPS2 || w3 < -EPS2 {
            None
        } else {
            Some(HitRec::new(
                w1 * self.vertex1 + w2 * self.vertex2 + w3 * self.vertex3,
                dot(e2, r),
            ))
        }
    }
}
