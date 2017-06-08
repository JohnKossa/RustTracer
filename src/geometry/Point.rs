use geometry::vector::Vector;
pub struct Point{
    x: i32,
    y: i32,
    z: i32
}

impl Point{
    pub fn origin() -> Point {
        Point {x:0, y:0, z:0}
    }

    pub fn new(x: i32, y: i32, z: i32) -> Point {
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
