#![allow(dead_code)]


use std::{cmp::max_by, vec};

use crate::matrix::{core::Vector, linear_combination, maxf, Matrix};

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
            rotation: Vector::from(&[0.0, 0.0, 90.0])
        }
    }

    pub fn move_forward(&mut self) {
        self.position = linear_combination(&[self.position.clone(), self.direction.clone()], &[1.0, 0.04]); // self.position = self.position + self.direction * 0.04
    }

    pub fn move_backward(&mut self) {
        self.position = linear_combination(&[self.position.clone(), self.direction.clone()], &[1.0, -0.04]); // self.position = self.position - self.direction * 0.04
    }

    pub fn move_left(&mut self) {
        let left = Vector::cross_product(&self.direction, &self.up);
        self.position =  linear_combination(&[self.position.clone(), left], &[1.0, 0.04]);
    }

    pub fn move_right(&mut self) {
        let right = Vector::cross_product(&self.up, &self.direction);
        self.position =  linear_combination(&[self.position.clone(), right], &[1.0, 0.04]);
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

        self.direction[0] = self.rotation[0].to_radians().sin();
        self.direction[1] = self.rotation[1] / 45.0;     
        self.direction[2] = self.rotation[0].to_radians().cos();
        self.direction.normalize();
        
        // self.up[1] = self.rotation[2].to_radians().sin();
        // self.up[0] = self.rotation[2].to_radians().cos();
        // self.up.normalize();
        println!("dir = {:?}, rot = {:?}", self.direction, self.rotation);
    }

    pub fn move_from_vector3(&mut self, vector: Vector<f32>, speed: f32) {
        self.position = linear_combination(&[self.position.clone(),
                                             Vector::cross_product(&self.up, &self.direction),
                                             self.up.clone(),
                                             self.direction.clone()],
                                           &[1.0,
                                             vector[0] * speed,
                                             vector[1] * speed,
                                             vector[2] * speed]); 
    }

}