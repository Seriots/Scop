#![allow(dead_code)]

use std::time::Instant;


pub struct FpsHandler {
    pub previous_frame: Instant,
    pub delta_time: f32,
}

impl FpsHandler {
    pub fn from_instant(start: Instant) -> Self {
        Self {previous_frame: start, delta_time: 0.0}
    }

    pub fn display_fps(&mut self, display: bool) -> f32{
        self.delta_time = (Instant::now() - self.previous_frame).as_secs_f32();
        let fps = 1.0 / self.delta_time;
        if display {
            println!("fps = {:.0}", fps);
        }
        self.previous_frame = Instant::now();
        fps
    }
}