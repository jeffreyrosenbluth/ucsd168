use crate::bvh::Node;
use crate::camera::Camera;
use crate::geom::{degrees_to_radians, point3, vec3, Color, Mat4, Point3, Vec3};
use crate::light::Light;
use crate::material::Material;
use crate::object::{Objects, Shape};
use crate::scene::World;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use std::sync::Arc;

pub struct Edsl {
    pub width: f32,
    pub height: f32,
    pub ambient: Vec3,
    pub objects: Objects,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub transforms: Vec<Mat4>,
    pub vertices: Vec<Vec3>,
    pub current_material: Material,
    pub attenuation: [f32; 3],
    pub max_depth: i32,
}

impl Edsl {
    pub fn new(
        width: f32,
        height: f32,
        ambient: Vec3,
        objects: Objects,
        lights: Vec<Light>,
        camera: Camera,
        transforms: Vec<Mat4>,
        vertices: Vec<Vec3>,
        current_material: Material,
        attenuation: [f32; 3],
        max_depth: i32,
    ) -> Self {
        Self {
            width,
            height,
            ambient,
            objects,
            lights,
            camera,
            transforms,
            vertices,
            current_material,
            attenuation,
            max_depth,
        }
    }

    pub fn size(&mut self, w: f32, h: f32) {
        self.width = w;
        self.height = h;
    }

    pub fn max_depth(&mut self, d: i32) {
        self.max_depth = d;
    }

    pub fn camera(&mut self, look_from: Point3, look_at: Point3, up: Point3, fov: f32) {
        self.camera = Camera::new(self.width, self.height, look_from, look_at, up, fov);
    }

    pub fn ambient(&mut self, r: f32, g: f32, b: f32) {
        let a = Color::new(r, g, b);
        self.ambient = a;
    }

    pub fn directional(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
        self.lights.push(Light::Directional { x, y, z, r, g, b });
    }

    pub fn point(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
        self.lights.push(Light::Point { x, y, z, r, g, b });
    }

    pub fn diffuse(&mut self, r: f32, g: f32, b: f32) {
        self.current_material.diffuse = Color::new(r, g, b);
    }

    pub fn attenuation(&mut self, c: f32, l: f32, q: f32) {
        self.attenuation = [c, l, q];
    }

    pub fn specular(&mut self, r: f32, g: f32, b: f32) {
        self.current_material.specular = Color::new(r, g, b);
    }

    pub fn shininess(&mut self, s: f32) {
        self.current_material.shininess = s;
    }

    pub fn emission(&mut self, r: f32, g: f32, b: f32) {
        self.current_material.emission = Color::new(r, g, b);
    }

    pub fn vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertices.push(point3(x, y, z));
    }

    pub fn sphere(&mut self, x: f32, y: f32, z: f32, r: f32) {
        let s = Sphere::new(
            point3(x, y, z),
            r,
            Arc::new(self.current_material.clone()),
            *self.transforms.last().unwrap(),
        );
        self.objects.0.push(Shape::Sphere(s));
    }

    pub fn tri(&mut self, a: usize, b: usize, c: usize) {
        let t = Triangle::new(
            self.vertices[a],
            self.vertices[b],
            self.vertices[c],
            Arc::new(self.current_material.clone()),
            *self.transforms.last().unwrap(),
        );
        self.objects.0.push(Shape::Triangle(t));
    }

    pub fn push(&mut self) {
        self.transforms.push(*self.transforms.last().unwrap());
    }

    pub fn pop(&mut self) {
        self.transforms.pop();
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let mat = Mat4::from_translation(vec3(x, y, z));
        let t = self.transforms.last_mut().unwrap();
        *t = mat * *t;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        let mat = Mat4::from_scale(vec3(x, y, z));
        let t = self.transforms.last_mut().unwrap();
        *t = mat * *t;
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32, a: f32) {
        let mat = Mat4::from_axis_angle(vec3(x, y, z), degrees_to_radians(a));
        let t = self.transforms.last_mut().unwrap();
        *t = mat * *t;
    }

    pub fn rotate_x(&mut self, a: f32) {
        let mat = Mat4::from_rotation_x(degrees_to_radians(a));
        let t = self.transforms.last_mut().unwrap();
        *t = mat * *t;
    }

    pub fn rotate_y(&mut self, a: f32) {
        let mat = Mat4::from_rotation_y(degrees_to_radians(a));
        let t = self.transforms.last_mut().unwrap();
        *t = mat * *t;
    }

    pub fn rotate_z(&mut self, a: f32) {
        let mat = Mat4::from_rotation_z(degrees_to_radians(a));
        let t = self.transforms.last_mut().unwrap();
        *t = mat * *t;
    }

    pub fn run(self) -> World {
        let indices: Vec<usize> = (0..self.objects.len()).collect();
        let nodes = Node::new(&self.objects, indices, 0);
        World {
            camera: self.camera,
            bvh_node: nodes,
            objects: self.objects,
            lights: self.lights,
            ambient: self.ambient,
            attenuation: self.attenuation,
            max_depth: self.max_depth,
        }
    }
}

impl Default for Edsl {
    fn default() -> Self {
        let camera = Camera::default();
        Self {
            width: camera.width,
            height: camera.height,
            ambient: Default::default(),
            objects: Default::default(),
            lights: Default::default(),
            camera,
            transforms: vec![Mat4::IDENTITY],
            vertices: Default::default(),
            current_material: Default::default(),
            attenuation: [1.0, 0.0, 0.0],
            max_depth: 5,
        }
    }
}
