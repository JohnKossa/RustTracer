use geometry::point::Point;
use scene_items::material::Material;
#[derive(Debug, Copy, Clone)]
pub struct Sphere{
    pub position: Point,
    pub radius: f64,
    pub material: Material
}
