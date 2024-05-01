#![allow(dead_code)]

pub struct Camera {
    pub position: [f32; 3],
    pub direction: [f32; 3],
    pub up: [f32; 3],
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, -2.0f32],
            direction: [0.0, 0.0, 1.0], 
            up: [0.0, 1.0, 0.0], 
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

    pub fn rotate_from_vector3(&mut self, vector: (f32, f32, f32), speed: f32) { 
        // self.direction[0] += vector.0.sin() * speed;
        // self.direction[2] += vector.0.cos() * speed;
        // println!("direction = {:?}", self.direction);
    }

    pub fn move_from_vector3(&mut self, vector: (f32, f32, f32), speed: f32) {
        self.position[0] += vector.0 * speed;
        self.position[1] += vector.1 * speed;
        self.position[2] += vector.2 * speed;
    }

}