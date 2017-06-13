use geometry::vector::Vector;
#[derive(Debug, Copy, Clone)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point{
    pub fn origin() -> Point {
        Point {x:0.0, y:0.0, z:0.0}
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point{x: x, y: y, z: z}
    }

    pub fn subtract_point(self, other: Point) -> Vector {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }

    pub fn add_vector(self, vector: Vector) -> Point {
        Point{x: self.x + vector.x, y: self.y + vector.y, z: self.z + vector.z}
    }

    pub fn subtract_vector(self, vector: Vector) -> Point {
        Point{x: self.x - vector.x, y: self.y - vector.y, z: self.z - vector.z}
    }
}
