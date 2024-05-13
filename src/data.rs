#![allow(dead_code)]

use std::time::Instant;

use glium::Texture2d;

use crate::{matrix::Vector, window::KeyEventHandler, Camera};


pub struct Data {
    pub start_time: Instant,
    pub camera: Camera,
    pub key_event_handler: KeyEventHandler,
    pub window_active: bool,
    pub window_extent: (f32, f32),
    pub object_position: (f32, f32, f32),
    pub color_mode: u16,
    pub transition_percent: f32,
    pub texture: Texture2d,
}

impl Data {
    pub fn new(window_extent: (f32, f32), texture: Texture2d) -> Self {
        Self {
                start_time: Instant::now(),
                camera: Camera::new(),
                key_event_handler: KeyEventHandler::new(),
                window_active: false,
                window_extent,
                object_position: (0.0, 0.0, 0.0),
                color_mode: 0,
                transition_percent: 1.0,
                texture,
            }
    }

    pub fn with_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.start_time = start_time;
        self
    }

    pub fn update_object_position(&mut self, position: Vector<f32>, speed: f32) {
        self.object_position = (self.object_position.0 + position[0] * speed, self.object_position.1 + position[1] * speed, self.object_position.2 + position[2] * speed);
    }

    pub fn update_color_mode(&mut self) {
        self.transition_percent = 0.0;
        self.color_mode = (self.color_mode + 1) % 3;
    }

    pub fn update_transition_percent(&mut self, delta_time: f32, speed: f32) {
        if self.transition_percent == 1.0 {
            return;
        }
        self.transition_percent += delta_time * speed;
        if self.transition_percent > 1.0 {
            self.transition_percent = 1.0;
        }
    }
}