#![allow(dead_code)]

use std::time::Instant;

use crate::Camera;



pub struct Data {
    pub start_time: Instant,
    pub camera: Camera,
}

impl Data {
    pub fn new() -> Self {
        Self {start_time: Instant::now(), camera: Camera::new() }
    }

    pub fn with_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.start_time = start_time;
        self
    }
}