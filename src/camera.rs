use crate::geom::*;
use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    tan_fovy_2: f32,
}

impl Camera {
    pub fn new(
        width: f32,
        height: f32,
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
        theta: f32,
    ) -> Self {
        let w = (look_from - look_at).normalize();
        let u = cross(up, w).normalize();
        let v = cross(w, u);
        let tan_fovy_2 = degrees_to_radians(theta / 2.0).tan();
        Self {
            width,
            height,
            look_from,
            look_at,
            up,
            u,
            v,
            w,
            tan_fovy_2,
        }
    }

    pub fn get_ray(&self, i: f32, j: f32) -> Ray {
        let origin = self.look_from;
        let tan_fovx_2 = self.tan_fovy_2 * self.width / self.height;
        let alpha = tan_fovx_2 * 2.0 / self.width * (j + 0.5 - self.width / 2.0);
        let beta = self.tan_fovy_2 * 2.0 / self.height * (self.height / 2.0 - i - 0.5);
        let direction = (alpha * self.u + beta * self.v - self.w).normalize();
        Ray { origin, direction }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            640.0,
            480.0,
            point3(0.0, 0.0, 0.0),
            point3(0.0, 0.0, 0.0),
            point3(0.0, 0.0, 0.0),
            30.0,
        )
    }
}
