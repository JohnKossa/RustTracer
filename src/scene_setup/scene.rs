use scene_items::sphere::Sphere;
use scene_items::light::Light;
use scene_items::color::Color;
use geometry::ray::Ray;
use geometry::point::Point;
use geometry::vector::Vector;

pub struct Scene{
    pub size_x: usize,
    pub size_y: usize,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>
}

pub fn hit_sphere_ray(sphere: Sphere, ray: Ray, t: f64) -> (bool, f64){
    let dist = sphere.position.subtract_point(ray.start);
    let b = ray.direction.dot(dist);
    let d = b.powf(2.0) - dist.dot(dist) + sphere.radius.powf(2.0);
    if d < 0.0 {
        return (false, -1.0)
    }
    let t0 = b - d.sqrt();
    let t1 = b + d.sqrt();
    if t1 > 0.1 && t0 < t {
        return (true, t0)
    }
    if t1 > 0.1 && t1 < t {
        return (true, t1)
    }
    (false, -1.0)
}

pub fn draw_scene(scene: Scene) -> Vec<Vec<Color>> {
    let mut to_return: Vec<Vec<Color>> = Vec::new();
    for y in 0..scene.size_y {
        to_return.append(&mut Vec::new());
        for x in 0..scene.size_x {
            let mut red = 0.0;
            let mut green = 0.0;
            let mut blue = 0.0;
            let mut coeff = 1.0;
            let mut level = 0;
            let mut view_ray = Ray {
                start: Point {x: x as f64, y: y as f64, z: -1000.0},
                direction: Vector {x: 0.0, y:0.0, z:1.0}
            };
            while coeff > 0.0 && level < 10 {
                let mut t = 2000.0;
                let mut current_sphere = -1;
                for i in 0..scene.spheres.len() {
                    let collision_test = hit_sphere_ray(scene.spheres[i], view_ray, t);
                    if collision_test.0 {
                        t = collision_test.1;
                        current_sphere = i as i32;
                    }
                }
                if current_sphere == -1{
                    break;
                }
                let new_start = view_ray.start.add_vector(view_ray.direction.scale(t));
                let mut normal = new_start.subtract_point(scene.spheres[current_sphere as usize].position);
                let mut temp = normal.dot(normal);
                if temp == 0.0 {
                    break;
                }
                temp = 1.0/temp.sqrt();
                normal = normal.scale(temp);
                let current_material = scene.spheres[current_sphere as usize].material;
                for light in scene.lights.iter() {
                    let dist = (*light).position.subtract_point(new_start);
                    if normal.dot(dist) <= 0.0{
                        continue;
                    }
                    let light_ray = Ray {start: new_start, direction: dist.scale(1.0/t)};
                    let mut in_shadow = false;
                    for sphere in scene.spheres.iter() {
                        let collision_test = hit_sphere_ray(*sphere, light_ray, t);
                        if collision_test.0 {
                            in_shadow = true;
                            break;
                        }
                    }
                    if !in_shadow {
                        let lambert = light_ray.direction.dot(normal) * coeff;
                        red += lambert * (*light).color.r * current_material.color.r;
                        green += lambert * (*light).color.g * current_material.color.g;
                        blue += lambert * (*light).color.b * current_material.color.b;
                    }
                }
                coeff = coeff * current_material.reflection;
                let reflect = 2.0 * view_ray.direction.dot(normal);
                view_ray.start = new_start;
                view_ray.direction = view_ray.direction.subtract(normal.scale(reflect));
                level += 1;
            }
            red = red.min(1.0);
            green = green.min(1.0);
            blue = blue.min(1.0);
            to_return[x][y] = Color {r: red, g: green, b: blue};
        }
    }
    return to_return;
}