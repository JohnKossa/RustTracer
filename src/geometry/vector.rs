#[derive(Debug, Copy, Clone)]
pub struct Vector{
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vector{
    pub fn origin() -> Vector {
        Vector {x:0, y:0, z:0}
    }

    pub fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector{x: x, y: y, z: z}
    }

    pub fn add(self, other: Vector) -> Vector {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }

    pub fn subtract(self, other: Vector) -> Vector {
        Vector{x: self.x-other.x, y: self.y-other.y, z: self.z-other.z}
    }

    pub fn dot(self, other: Vector) -> i32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn scale(self, factor: f64) -> Vector {
        Vector{x: (self.x as f64 * factor) as i32, y: (self.y as f64 * factor) as i32, z: (self.z as f64 * factor) as i32}
    }
}