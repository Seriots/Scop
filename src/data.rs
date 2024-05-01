#![allow(dead_code)]

use std::time::Instant;

pub struct Data {
    pub start_time: Instant,
}

impl Data {
    pub fn new() -> Self {
        Self {start_time: Instant::now() }
    }

    pub fn with_start_time(&mut self, start_time: Instant) -> &mut Self {
        self.start_time = start_time;
        self
    }
}