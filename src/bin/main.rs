use std::path::PathBuf;
use ucsd168::parse::*;
use ucsd168::render::*;
use ucsd168::io::write_png;

pub fn main() {
    let world = parse_scene(PathBuf::from(
        "/Users/jeffreyrosenbluth/Rust/ucsd168/testscenes/scene4-diffuse.test",
    )).unwrap();
    let data = render(&world);
    write_png(&data, world.camera.width as u32, world.camera.height as u32, "image")
}
