use crate::aabb::Aabb;
use crate::geom::{cross, dot, Point3, Ray};
use crate::material::Material;
use crate::object::Hit;
use glam::{vec3, Mat4};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertex1: Point3,
    pub vertex2: Point3,
    pub vertex3: Point3,
    pub material: Arc<Material>,
    pub transform: Mat4,
    pub inv_transform: Mat4,
    pub bounding_box: Aabb,
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
            bounding_box: Self::bounding_box(vertex1, vertex2, vertex3),
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
            let n = self
                .inv_transform
                .transpose()
                .transform_vector3(cross(e1, e2));
            Some(Hit::new(
                self.transform.transform_point3(p),
                t,
                n.normalize(),
                self.material.clone(),
            ))
        }
    }

    pub fn bounding_box(v1: Point3, v2: Point3, v3: Point3) -> Aabb {
        let x_min = v1.x.min(v2.x).min(v3.x);
        let x_max = v1.x.max(v2.x).max(v3.x);
        let y_min = v1.y.min(v2.y).min(v3.y);
        let y_max = v1.y.max(v2.y).max(v3.y);
        let z_min = v1.z.min(v2.z).min(v3.z);
        let z_max = v1.z.max(v2.z).max(v3.z);
        Aabb::new(vec3(x_min, y_min, z_min), vec3(x_max, y_max, z_max))
    }
}
