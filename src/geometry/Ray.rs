use geometry::point::Point;
use geometry::vector::Vector;
#[derive(Debug, Copy, Clone)]
pub struct Ray{
    pub start: Point,
    pub direction: Vector
}