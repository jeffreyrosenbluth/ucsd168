use crate::camera::Camera;
use crate::geom::*;
use crate::light::Light;
use crate::object::Objects;

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub objects: Objects,
    pub lights: Vec<Light>,
    pub ambient: Color,
}
