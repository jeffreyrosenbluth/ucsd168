use crate::bvh::Node;
use crate::camera::Camera;
use crate::geom::*;
use crate::light::Light;
use crate::object::Objects;

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub bvh_node: Node,
    pub objects: Objects,
    pub lights: Vec<Light>,
    pub ambient: Color,
    pub attenuation: [f32; 3],
    pub max_depth: i32,
}
