struct Vector{
    x: i32,
    y: i32,
    z: i32
}

impl Vector{
    fn origin() -> Vector {
        Vector {x:0, y:0, z:0}
    }

    fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector{x: x, y: y, z: z}
    }

    fn add(self, other: Vector) -> Vector {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }

    fn subtract(self, other: Vector) -> Vector {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }

    fn dot(self, other: Vector) -> i32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    fn scale(self, factor: f32) -> Vector {
        Vector{x: self.x as f32 * factor, y: self.y as f32 * factor, z: self.z as f32 * factor}
    }
}