#![allow(dead_code)]


use crate::matrix::{core::Vector, linear_combination};

pub struct Camera {
    pub position: Vector<f32>,
    pub direction: Vector<f32>,
    pub up: Vector<f32>,
    pub rotation: Vector<f32>
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector::from(&[0.0, 0.0, -2.0f32]),
            direction: Vector::from(&[0.0, 0.0, 1.0]), 
            up: Vector::from(&[0.0, 1.0, 0.0]), 
            rotation: Vector::from(&[0.0, 90.0, 0.0])
        }
    }

    pub fn move_forward(&mut self) {
        self.position[2] += 0.04;
    }

    pub fn move_backward(&mut self) {
        self.position[2] -= 0.04;
    }

    pub fn move_left(&mut self) {
        self.position[0] -= 0.04;
    }

    pub fn move_right(&mut self) {
        self.position[0] += 0.04;
    }

    pub fn move_up(&mut self) {
        self.position[1] += 0.04;
    }

    pub fn move_down(&mut self) {
        self.position[1] -= 0.04;
    }

    pub fn rotate_from_vector3(&mut self, vector: Vector<f32>, speed: f32) {
        self.rotation = linear_combination(&[self.rotation.clone(), vector], &[1.0, speed]); // self.rotation = self.rotation + vector * speed
        self.rotation[1] = self.rotation[1].clamp(-180.0, 180.0);
        self.rotation[0] = self.rotation[0].rem_euclid(360.0);

        // let mut v1 = Vector::from(&[self.rotation[0].sin(), 1.0, self.rotation[0].cos()]);
        // let v2 = Vector::from(&[self.rotation[1].cos(), self.rotation[1].sin(), 0.0]);

        // v1.add(&v2);
        // self.direction = v1;

        // self.direction[0] = self.rotation[0].to_radians().sin();
        // self.direction[1] = if self.rotation[1].to_radians().cos() < self.rotation[1].to_radians().sin() { self.rotation[1].to_radians().cos() } else { self.rotation[1].to_radians().sin() };     
        // self.direction[2] = self.rotation[0].to_radians().cos();
        
        self.direction.normalize();
        println!("{:?}", self.direction);
    }

    pub fn move_from_vector3(&mut self, vector: Vector<f32>, speed: f32) {
        self.position = linear_combination(&[self.position.clone(), vector], &[1.0, speed]); // self.position = self.position + vector * speed
    }

}