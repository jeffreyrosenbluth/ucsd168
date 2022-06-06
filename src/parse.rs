use crate::bvh::Node;
use crate::camera::*;
use crate::geom::*;
use crate::light::*;
use crate::material::*;
use crate::object::*;
use crate::scene::*;
use crate::shapes::sphere::Sphere;
use crate::shapes::triangle::Triangle;
use anyhow::{anyhow, Result};
use glam::{vec3, Mat4};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub fn parse_scene<'a>(path: PathBuf) -> Result<World> {
    let mut w = 0.0;
    let mut h = 0.0;
    let mut ambient = BLACK;
    let mut objects = Objects(Vec::new());
    let mut lights = Vec::new();
    let mut camera = Camera::default();
    let mut material = Material::default();
    let mut _maxverts = 0;
    let mut vertices = Vec::new();
    let mut transforms: Vec<Mat4> = vec![Mat4::IDENTITY];
    let mut attenuation = [1.0, 0.0, 0.0];
    let mut max_depth = 5;

    let scene = fs::read_to_string(path)?;
    let lines = scene.lines();
    for line in lines {
        let tokens: Vec<&str> = line.split_ascii_whitespace().collect();
        if tokens.is_empty() || tokens[0] == "#" {
            continue;
        };
        match tokens[0] {
            "size" => {
                if tokens.len() != 3 {
                    return Err(anyhow!(
                        "size command requires 2 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                w = tokens[1].parse::<f32>()?;
                h = tokens[2].parse::<f32>()?;
            }
            "maxdepth" => {
                if tokens.len() != 2 {
                    return Err(anyhow!(
                        "size command requires 1 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                max_depth = tokens[1].parse::<i32>()?;
            }
            "camera" => {
                if tokens.len() != 11 {
                    return Err(anyhow!(
                        "camera command requires 10 arguments, not {}",
                        tokens.len() - 1,
                    ));
                };
                let from_x = tokens[1].parse::<f32>()?;
                let from_y = tokens[2].parse::<f32>()?;
                let from_z = tokens[3].parse::<f32>()?;
                let at_x = tokens[4].parse::<f32>()?;
                let at_y = tokens[5].parse::<f32>()?;
                let at_z = tokens[6].parse::<f32>()?;
                let up_x = tokens[7].parse::<f32>()?;
                let up_y = tokens[8].parse::<f32>()?;
                let up_z = tokens[9].parse::<f32>()?;
                let fov = tokens[10].parse::<f32>()?;
                let look_from = point3(from_x, from_y, from_z);
                let look_at = point3(at_x, at_y, at_z);
                let up = point3(up_x, up_y, up_z);
                camera = Camera::new(w, h, look_from, look_at, up, fov);
            }
            "ambient" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "ambient command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let r = tokens[1].parse::<f32>()?;
                let g = tokens[2].parse::<f32>()?;
                let b = tokens[3].parse::<f32>()?;
                ambient = Color::new(r, g, b);
            }
            "directional" => {
                if tokens.len() != 7 {
                    return Err(anyhow!(
                        "directional command requires 6 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                let r = tokens[4].parse::<f32>()?;
                let g = tokens[5].parse::<f32>()?;
                let b = tokens[6].parse::<f32>()?;
                lights.push(Light::Directional { x, y, z, r, g, b });
            }
            "point" => {
                if tokens.len() != 7 {
                    return Err(anyhow!(
                        "point command requires 6 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                let r = tokens[4].parse::<f32>()?;
                let g = tokens[5].parse::<f32>()?;
                let b = tokens[6].parse::<f32>()?;
                lights.push(Light::Point { x, y, z, r, g, b });
            }
            "attenuation" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "attenuation command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let c = tokens[1].parse::<f32>()?;
                let l = tokens[2].parse::<f32>()?;
                let q = tokens[3].parse::<f32>()?;
                attenuation = [c, l, q];
            }
            "diffuse" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "diffuse command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let r = tokens[1].parse::<f32>()?;
                let g = tokens[2].parse::<f32>()?;
                let b = tokens[3].parse::<f32>()?;
                material.diffuse = Color::new(r, g, b);
            }
            "specular" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "specular command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let r = tokens[1].parse::<f32>()?;
                let g = tokens[2].parse::<f32>()?;
                let b = tokens[3].parse::<f32>()?;
                material.specular = Color::new(r, g, b);
            }
            "shininess" => {
                if tokens.len() != 2 {
                    return Err(anyhow!(
                        "shininess command requires 1 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let s = tokens[1].parse::<f32>()?;
                material.shininess = s;
            }
            "emission" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "emission command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let r = tokens[1].parse::<f32>()?;
                let g = tokens[2].parse::<f32>()?;
                let b = tokens[3].parse::<f32>()?;
                material.emission = Color::new(r, g, b);
            }
            "maxverts" => {
                if tokens.len() != 2 {
                    return Err(anyhow!(
                        "maxverts command requires 1 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                _maxverts = tokens[1].parse::<u32>()?;
            }
            "vertex" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "vertex command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                vertices.push(point3(x, y, z));
            }
            "tri" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "tri command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<usize>()?;
                let y = tokens[2].parse::<usize>()?;
                let z = tokens[3].parse::<usize>()?;
                let triangle = Triangle::new(
                    vertices[x],
                    vertices[y],
                    vertices[z],
                    Arc::new(material.clone()),
                    *transforms.last().unwrap(),
                );
                objects.0.push(Shape::Triangle(triangle));
            }
            "sphere" => {
                if tokens.len() != 5 {
                    return Err(anyhow!(
                        "sphere command requires 4 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                let r = tokens[4].parse::<f32>()?;
                let sphere = Sphere::new(
                    point3(x, y, z),
                    r,
                    Arc::new(material.clone()),
                    *transforms.last().unwrap(),
                );
                objects.0.push(Shape::Sphere(sphere));
            }
            "pushTransform" => transforms.push(*transforms.last().unwrap()),
            "popTransform" => {
                transforms.pop();
            }
            "translate" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "translate command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                let mat = Mat4::from_translation(vec3(x, y, z));
                let t = transforms.last_mut().unwrap();
                *t *= mat;
            }
            "scale" => {
                if tokens.len() != 4 {
                    return Err(anyhow!(
                        "scale command requires 3 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                let mat = Mat4::from_scale(vec3(x, y, z));
                let t = transforms.last_mut().unwrap();
                *t *= mat;
            }
            "rotate" => {
                if tokens.len() != 5 {
                    return Err(anyhow!(
                        "rotate command requires 4 arguments, not {}",
                        tokens.len() - 1
                    ));
                };
                let x = tokens[1].parse::<f32>()?;
                let y = tokens[2].parse::<f32>()?;
                let z = tokens[3].parse::<f32>()?;
                let a = tokens[4].parse::<f32>()?;
                let mat = Mat4::from_axis_angle(vec3(x, y, z), degrees_to_radians(a));
                let t = transforms.last_mut().unwrap();
                *t *= mat;
            }
            _ => continue,
        }
    }
    let n = objects.0.len();
    let nodes = Node::new(&mut objects, 0, n, 0);

    Ok(World {
        camera,
        objects: nodes,
        lights,
        ambient,
        attenuation,
        max_depth,
    })
}
