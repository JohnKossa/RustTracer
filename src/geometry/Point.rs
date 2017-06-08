struct Point{
    x: i32,
    y: i32,
    z: i32
}

impl Point{
    fn origin() -> Point {
        Point {x:0, y:0, z:0}
    }

    fn new(x: i32, y: i32, z: i32) -> Point {
        Point{x: x, y: y, z: z}
    }

    fn subtract_point(self, other: Point) -> Point {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }

    fn add_vector(self, vector: Vector) -> Point {
        Point{x: self.x + vector.x, y: self.y + vector.y, z: self.z + vector.z}
    }

    fn subtract_vector(self, other: Vector) -> Point {
        Point{x: self.x - vector.x, y: self.y - vector.y, z: self.z - vector.z}
    }
}