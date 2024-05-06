#![allow(dead_code)]


use crate::{matrix::{core::Vector, linear_combination}, Quaternion};

pub struct Camera {
    pub position: Vector<f32>,
    pub direction: Vector<f32>,
    pub up: Vector<f32>,
    pub rotation: Vector<f32>,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector::from(&[0.0, 0.0, -2.0f32]),
            direction: Vector::from(&[0.0, 0.0, 1.0]), 
            up: Vector::from(&[0.0, 1.0, 0.0]),
            rotation: Vector::from(&[0.0, 0.0, 90.0]),
        }
    }

    pub fn project_direction_2d(&self) -> Vector<f32> {
        let mut direction = self.direction.clone();
        direction[1] = 0.0;
        direction.normalize();
        direction
    }

    pub fn rotate_from_vector3(&mut self, vector: Vector<f32>, speed: f32) {

        let mut rotation = vector.clone();
        rotation.scl(speed);

        self.rotation.add(&rotation);
        self.rotation[1] = self.rotation[1].clamp(-89.9, 89.9);
        self.rotation[0] = self.rotation[0].rem_euclid(360.0);

        //rotate around y axis
        let p = Quaternion::from_vec(&Vector::from(&[0.0, 0.0, 1.0]));
        
        let mut q = Quaternion::from_angle(self.rotation[0].to_radians(), Vector::from(&[0.0, 1.0, 0.0]));
        q.normalize();
        let q_inv = q.inverse();

        let p_rot = q * p * q_inv;

        self.direction = Vector::from(&[p_rot.x, p_rot.y, p_rot.z]);
        
        //rotate around x axis
        let p = Quaternion::from_vec(&self.direction);
        let mut u = Vector::cross_product(&self.direction, &self.up);
        u.normalize();
        
        let mut q = Quaternion::from_angle(self.rotation[1].to_radians(), u);
        q.normalize();
        let q_inv = q.inverse();
        
        let p_rot = q * p * q_inv;
        
        self.direction = Vector::from(&[p_rot.x, p_rot.y, p_rot.z]);
        
    }

    pub fn move_from_vector3(&mut self, vector: Vector<f32>, speed: f32) {
        let direction2d = self.project_direction_2d(); 
        self.position = linear_combination(&[self.position.clone(),
                                             Vector::cross_product(&self.up, &direction2d),
                                             self.up.clone(),
                                             direction2d],
                                           &[1.0,
                                             vector[0] * speed,
                                             vector[1] * speed,
                                             vector[2] * speed]); 
    }

}