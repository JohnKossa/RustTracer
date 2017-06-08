fn hit_sphere_ray(sphere: Sphere, ray: ray, t: i32) -> (bool, f32){
    let dist = sphere.position.subtract_point(ray.start);
    let B = ray.direction.dot(dist);
    let D = B.pow(2) - dist.dot(dist) + sphere.radius.pow(2);
    if D < 0{
        return (false, -1.0_f32)
    }
    let t0 = (B as f32) - (D as f32).sqrt();
    let t1 = (B as f32) + (D as f32).sqrt();
    if (t1 > 0.1) && (t0 < t) {
        return (true, t0)
    }
    if (t1 > 0.1) && (t1 < t) {
        return (true, t1)
    }
    (false, -1.0_f32)
}