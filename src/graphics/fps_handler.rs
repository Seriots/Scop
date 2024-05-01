#![allow(dead_code)]

use std::time::Instant;


pub struct FpsHandler {
    pub previous_frame: Instant,
}

impl FpsHandler {
    pub fn from_instant(start: Instant) -> Self {
        Self {previous_frame: start}
    }

    pub fn display_fps(&mut self, display: bool) -> f32{
        let fps = 1.0 / (Instant::now() - self.previous_frame).as_secs_f32();
        if display {
            println!("fps = {:.0}", fps);
        }
        self.previous_frame = Instant::now();
        fps
    }
}