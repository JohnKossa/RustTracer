use scene_items::sphere::Sphere;
use scene_items::light::Light;
use geometry::ray::Ray;
pub struct Scene{
    pub size_x: i32,
    pub size_y: i32,
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>
}

pub fn hit_sphere_ray(sphere: Sphere, ray: Ray, t: i32) -> (bool, f32){
    let dist = sphere.position.subtract_point(ray.start);
    let b = ray.direction.dot(dist);
    let d = b.pow(2) - dist.dot(dist) + sphere.radius.pow(2);
    if d < 0{
        return (false, -1.0)
    }
    let t0 = (b as f32) - (d as f32).sqrt();
    let t1 = (b as f32) + (d as f32).sqrt();
    if (t1 > 0.1) && (t0 < t as f32) {
        return (true, t0)
    }
    if (t1 > 0.1) && (t1 < t as f32) {
        return (true, t1)
    }
    (false, -1.0)
}