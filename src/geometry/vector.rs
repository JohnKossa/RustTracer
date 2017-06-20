use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector{
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector{
    pub fn origin() -> Vector {
        Vector {x:0.0, y:0.0, z:0.0}
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector{x: x, y: y, z: z}
    }

    pub fn dot(self, other: Vector) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn scale(self, factor: f64) -> Vector {
        Vector{x: (self.x * factor), y: (self.y * factor), z: (self.z * factor)}
    }
}

impl Add for Vector{
    type Output = Vector;

    fn add(self, other:Vector) -> Vector{
        Vector{x:self.x+other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vector{
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector{
        Vector{x:self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}