use crate::geom::*;
use crate::object::*;
use crate::parse::*;
use crate::scene::*;
use rayon::prelude::*;

pub fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.objects.hit(ray, 0.001, f32::MAX) {
        intensity(ray, &rec, world)
    } else {
        BLACK
    }
}

pub fn intensity(wi: &Ray, rec: &Hit, world: &World) -> Color {
    let mut color = BLACK;
    color += world.ambient + rec.material.emission;
    for light in &world.lights {
        match light {
            Light::Directional { x, y, z, r, g, b } => {
                let light_vector = -vec3(*x, *y, *z);
                let light_direction = light_vector.normalize();
                let light_ray = Ray::new(rec.point, light_direction);
                let h = ((wi.origin - rec.point) + light_vector).normalize();
                let hit = world.objects.hit(&light_ray, 0.001, f32::MAX);
                if hit.is_none() {
                    color += Color::new(*r, *g, *b)
                        * rec.material.diffuse
                        * dot(rec.normal, light_direction).max(0.0)
                        + rec.material.specular
                            * dot(rec.normal, h).max(0.0).powf(rec.material.shininess)
                }
            }
            Light::Point { x, y, z, r, g, b } => {
                let light_position = point3(*x, *y, *z);
                let light_vector = light_position - rec.point;
                let light_direction = light_vector.normalize();
                let light_ray = Ray::new(rec.point, light_direction);
                let h = ((wi.origin - rec.point) + light_vector).normalize();
                let hit = world.objects.hit(&light_ray, 0.001, f32::MAX);
                if hit.is_none() || hit.unwrap().t > light_vector.length() {
                    color += (Color::new(*r, *g, *b)
                        * rec.material.diffuse
                        * dot(rec.normal, light_direction).max(0.0)
                        + rec.material.specular
                            * dot(rec.normal, h).max(0.0).powf(rec.material.shininess))
                        / light_vector.length_squared();
                }
            }
        }
    }
    color
}

fn write_color(data: &mut Vec<u8>, pixel_color: Color, samples_per_pixel: u32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as Float;
    // May want to gamma correct here.
    r = scale * r;
    g = scale * g;
    b = scale * b;

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
