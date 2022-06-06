use std::path::PathBuf;
use ucsd168::edsl::Edsl;
use ucsd168::geom::point3;
use ucsd168::io::write_png;
use ucsd168::parse::*;
use ucsd168::render::*;
use ucsd168::scene::World;

pub fn main() {
    let world = parse_scene(PathBuf::from(
        "/Users/jeffreyrosenbluth/Develop/ucsd168/testscenes/scene7.test",
    ))
    .unwrap();
    println!("World loaded");
    // let world = scene_1();
    let data = render(&world);
    write_png(
        &data,
        world.camera.width as u32,
        world.camera.height as u32,
        "image",
    )
}

pub fn scene_1() -> World {
    let mut scene = Edsl::default();
    scene.size(640.0, 480.0);
    scene.camera(
        point3(0.0, 0.0, 1.0),
        point3(0.0, 0.0, 0.0),
        point3(0.0, 1.0, 0.0),
        30.0,
    );
    scene.directional(0.0, -1.0, 0.0, 1.0, 1.0, 1.0);
    scene.point(-4.0, -1.0, -4.0, 12.0, 12.0, 12.0);
    scene.ambient(0.18, 0.04, 0.18);
    scene.diffuse(0.9, 0.2, 0.9);
    scene.specular(1.0, 1.0, 1.0);
    scene.shininess(200.0);
    scene.sphere(0.0, -1.0, -4.0, 1.0);
    scene.push();
    scene.scale(0.3, 0.15, 0.15);
    scene.rotate(0.0, 1.0, 1.0, 45.0);
    scene.translate(0.0, 0.5, -4.0);
    scene.sphere(0.0, 0.0, 0.0, 1.0);
    scene.pop();
    scene.push();
    scene.run()
}
