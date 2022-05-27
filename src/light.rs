#[derive(Debug, Clone, Copy)]
pub enum Light {
    Directional {
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        g: f32,
        b: f32,
    },
    Point {
        x: f32,
        y: f32,
        z: f32,
        r: f32,
        g: f32,
        b: f32,
    },
}