use std::ops::{Add};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z}
    }

    pub fn dot(self, rhs: Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }

    pub fn normalize(self) -> Option<Vector3> {
        let norm: f64 = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        
        if norm < 1e-12{
            return None;
        }
        
        Some(Vector3 {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm
        })
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}