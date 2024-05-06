#![allow(dead_code)]

use std::time::Instant;

use crate::{window::KeyEventHandler, Camera};


pub struct Data {
    pub start_time: Instant,
    pub camera: Camera,
    pub key_event_handler: KeyEventHandler,
    pub window_active: bool,
    pub window_extent: (f32, f32),
}

impl Data {
    pub fn new(window_extent: (f32, f32)) -> Self {
        Self {start_time: Instant::now(), camera: Camera::new(), key_event_handler: KeyEventHandler::new(), window_active: false, window_extent}
    }

    pub fn with_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.start_time = start_time;
        self
    }
}