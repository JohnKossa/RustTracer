extern crate sdl2;
extern crate gl;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::rect::Rect;

mod geometry;
mod scene_items;
mod scene_setup;
use geometry::point::Point;
use scene_setup::scene::{Scene, draw_scene};
use scene_items::color::Color;
use scene_items::sphere::Sphere;
use scene_items::light::Light;
use scene_items::material::Material;

fn build_scene() -> Scene{
    let mut my_scene = Scene{size_x: 800, size_y: 600, spheres: Vec::new(), lights: Vec::new()};
    my_scene.spheres.push(Sphere::new(Point::new(100.0, 500.0, 0.0), 100.0, Material::new(0.95, Color::MAGENTA().scale(0.3))));
    my_scene.spheres.push(Sphere::new(Point::new(500.0, 500.0, 0.0), 100.0, Material::new(0.95, Color::GREEN().scale(0.3))));
    my_scene.spheres.push(Sphere::new(Point::new(100.0, 100.0, 0.0), 100.0, Material::new(0.95, Color::BLUE().scale(0.3))));
    my_scene.spheres.push(Sphere::new(Point::new(500.0, 100.0, 0.0), 100.0, Material::new(0.95, Color::YELLOW().scale(0.3))));
    my_scene.spheres.push(Sphere::new(Point::new(300.0, 300.0, 100.0), 100.0, Material::new(0.0, Color::WHITE())));
    my_scene.lights.push(Light::new(Point::new(500.0, -1000.0, -100.0), Color::new(0.9, 0.5, 0.5)));
    my_scene.lights.push(Light::new(Point::new(640.0, 240.0, -10000.0), Color::new(0.5, 0.5, 0.9)));
    my_scene
}

fn main() {
    let my_scene = build_scene();
    let my_colors = draw_scene(my_scene);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().software().build().unwrap();
    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut events = sdl_context.event_pump().unwrap();
    'mainloop: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }
        canvas.clear();
        let y_size = my_colors.len()-1;
        let x_size = my_colors[0].len()-1;
        for x in 0..x_size{
            for y in 0..y_size {
                canvas.set_draw_color(pixels::Color::RGB((my_colors[y][x].r * 255.0) as u8, (my_colors[y][x].g * 255.0) as u8, (my_colors[y][x].b * 255.0) as u8));
                canvas.fill_rect(Rect::new(x as i32, y as i32, 1, 1));
                //println!("{},{}",x, y);
            }
        }

        //canvas.set_draw_color(pixels::Color::RGB(255, 210, 0));
        //canvas.fill_rect(Rect::new(10, 10, 780, 580));
        canvas.present();
    }

    //gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    //canvas.window().gl_set_context_to_current();
    //let tmp = Scene{size_x: 200, size_y: 200, spheres: Vec::new(), lights: Vec::new()};
    //println!("Hello, world!");
}
