mod geometry;
mod scene_items;
mod scene_setup;
use scene_setup::scene::Scene;

fn main() {
    let tmp = Scene{size_x: 200, size_y: 200, spheres: Vec::new(), lights: Vec::new()};
    println!("Hello, world!");
}
