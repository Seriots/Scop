use std::ops::Mul;

use crate::matrix::Vector;


pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_vec(vec3: &Vector<f32>) -> Self {
        Self { x: vec3[0], y: vec3[1], z: vec3[2], w: 0.0 }
    }

    pub fn from_angle(angle: f32, vec3: Vector<f32>) -> Self {
        Self {
            x: (angle / 2.0).sin() * vec3[0],
            y: (angle / 2.0).sin() * vec3[1],
            z: (angle / 2.0).sin() * vec3[2],
            w: (angle / 2.0).cos(),
        }
    }

    pub fn normalize(&mut self) {
        let len = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        let len = len.sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self.w /= len;
    }

    pub fn inverse(&self) -> Self {
        let len = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        Self {
            x: -self.x / len,
            y: -self.y / len,
            z: -self.z / len,
            w: self.w / len,
        }
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}