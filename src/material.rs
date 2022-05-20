use crate::geom::Color;

#[derive(Debug, Clone, Copy)]

pub struct Material {
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f32,
    pub emission: Color,
}

impl Material {
    pub fn new(diffuse: Color, specular: Color, shininess: f32, emission: Color) -> Self {
        Self {
            diffuse,
            specular,
            shininess,
            emission,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse: Color::new(0.0, 0.0, 0.0),
            specular: Color::new(0.0, 0.0, 0.0),
            shininess: 0.0,
            emission: Color::new(0.0, 0.0, 0.0),
        }
    }
}
