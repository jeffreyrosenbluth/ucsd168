use std::path::PathBuf;
use uscd168::parse::*;
use uscd168::render::*;
use uscd168::io::write_png;

pub fn main() {
    let world = parse_scene(PathBuf::from(
        "/Users/jeffreyrosenbluth/Rust/uscd168/testscenes/scene1.test",
    )).unwrap();
    let data = render(&world);
    write_png(&data, world.camera.width as u32, world.camera.height as u32, "imgage")
}
