#![allow(dead_code)]

use std::time::Instant;

use crate::{matrix::Vector, window::KeyEventHandler, Camera};


pub struct Data {
    pub start_time: Instant,
    pub camera: Camera,
    pub key_event_handler: KeyEventHandler,
    pub window_active: bool,
    pub window_extent: (f32, f32),
    pub object_position: (f32, f32, f32),
}

impl Data {
    pub fn new(window_extent: (f32, f32)) -> Self {
        Self {start_time: Instant::now(), camera: Camera::new(), key_event_handler: KeyEventHandler::new(), window_active: false, window_extent, object_position: (0.0, 0.0, 0.0)}
    }

    pub fn with_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.start_time = start_time;
        self
    }

    pub fn update_object_position(&mut self, position: Vector<f32>, speed: f32) {
        self.object_position = (self.object_position.0 + position[0] * speed, self.object_position.1 + position[1] * speed, self.object_position.2 + position[2] * speed);
    }
}