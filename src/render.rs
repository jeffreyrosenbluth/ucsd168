use crate::geom::*;
use crate::parse::*;
use rayon::prelude::*;

pub fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(_rec) = world.objects.hit(ray, 0.001, f32::MAX) {
        color(0.85, 0.2, 0.2)
    } else {
        BLACK
    }
}

fn write_color(data: &mut Vec<u8>, pixel_color: Color, samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as Float;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    data.push((255.999 * r) as u8);
    data.push((255.999 * g) as u8);
    data.push((255.999 * b) as u8);
}

pub fn render(environment: &World) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let w = environment.camera.width as u32;
    let h = environment.camera.height as u32;

    for j in 0..h {
        eprintln!("Scanlines remaining: {}", j + 1);
        let scanline: Vec<Color> = (0..w)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = BLACK;
                let r = environment.camera.get_ray(j as f32, i as f32);
                let mut rc = ray_color(&r, environment);
                if rc.x.is_nan() {
                    rc.x = 0.0
                };
                if rc.y.is_nan() {
                    rc.y = 0.0
                };
                if rc.z.is_nan() {
                    rc.z = 0.0
                };
                pixel_color += rc;
                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            write_color(&mut data, pixel_color, 1);
        }
    }
    data
}
